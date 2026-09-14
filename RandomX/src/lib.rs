//! An educational Rust interface to **RandomX**, the proof-of-work hash
//! algorithm used by Monero since November 2019.
//!
//! Complete hashes use the maintained RandomX reference backend in light
//! mode. The independently useful Argon2d cache, AES generators, dataset
//! register seeding, and reciprocal calculation remain visible as Rust
//! modules with official reference-vector tests.
//!
//! # Anatomy of one `calculate_hash(key, input)` call
//!
//! 1. **Cache** ([`cache`]) — derive 256 MiB of pseudo-random data from
//!    `key` using Argon2d. Expensive, but only needs to happen once per key
//!    (Monero changes the key roughly every 2048 blocks), not once per hash.
//! 2. **Superscalar programs** — derive 8 small integer programs from `key`
//!    used to expand Cache reads into full Dataset items on demand.
//! 3. **Seed the first program** — hash `input` (and a bit of fixed
//!    context) to produce the initial 128-byte entropy buffer.
//! 4. **Run 8 chained programs** in the native RandomX VM: for each of
//!    `RANDOMX_PROGRAM_COUNT` (8) rounds:
//!    a. Decode a [`program::Program`] (256 instructions) from the current
//!       entropy buffer.
//!    b. Feed the VM's integer/float registers from the same entropy
//!       buffer, and AES-fill the 2 MiB scratchpad ([`aes_generator`]).
//!    c. Execute the program's instruction loop `RANDOMX_PROGRAM_ITERATIONS`
//!       (2048) times. Instructions read/write the scratchpad, and every
//!       memory-hard access pulls one 64-byte item from the Dataset
//!       ([`dataset`]), which is derived from the Cache via the
//!       superscalar programs.
//!    d. "Hash and fill": mix the register file into the scratchpad, which
//!       both updates the scratchpad and produces the entropy buffer that
//!       seeds the *next* program.
//! 5. **Finalize** — after the 8th program, Blake2b-hash the final register
//!    file into a 32-byte digest. That digest *is* the RandomX hash.
//!
//! Steps 1-2 depend only on `key`; in a real miner they'd be computed once
//! and reused across many hashes. Steps 3-5 run fresh for every `input`.

pub mod aes_generator;
pub mod cache;
pub mod dataset;
pub mod params;
pub mod reciprocal;
pub mod telemetry;

use randomx_rs::{RandomXCache, RandomXFlag, RandomXVM};
use telemetry::HashTelemetry;

/// Compute a single RandomX hash of `input` under key `key`, in light mode.
///
/// This mirrors the reference implementation's
/// `randomx_init_cache` + `randomx_create_vm` + `randomx_calculate_hash`
/// sequence, collapsed into one call for a from-scratch, "compute one hash
/// end to end" first pass.
pub fn calculate_hash(key: &[u8], input: &[u8]) -> [u8; params::HASH_SIZE] {
    calculate_hash_with_telemetry(key, input).0
}

/// Same as [`calculate_hash`], but also returns a [`HashTelemetry`] trace
/// with a high-resolution timing for every pipeline stage listed in the
/// module docs above (Cache init, SuperscalarHash generation, entropy
/// seeding, per-program generate/AES-fill/execute/hash-and-fill, and final
/// Blake2b), in the order they ran.
pub fn calculate_hash_with_telemetry(
    key: &[u8],
    input: &[u8],
) -> ([u8; params::HASH_SIZE], HashTelemetry) {
    assert!(!key.is_empty(), "RandomX key must not be empty");
    assert!(!input.is_empty(), "RandomX input must not be empty");

    let mut telemetry = HashTelemetry::new();
    let flags = RandomXFlag::get_recommended_flags();
    let cache_bytes = params::ARGON_MEMORY_KIB as usize * 1024;
    let cache = telemetry.time_with_memory("cache_init", cache_bytes, || {
        RandomXCache::new(flags, key).expect("failed to initialize RandomX cache")
    });
    let vm = telemetry.time_with_memory("vm_init", params::SCRATCHPAD_L3, || {
        RandomXVM::new(flags, Some(cache), None).expect("failed to initialize RandomX VM")
    });
    let hash = telemetry.time_with_memory(
        "hash_execute",
        cache_bytes + params::SCRATCHPAD_L3,
        || vm.calculate_hash(input).expect("RandomX hash calculation failed"),
    );

    (
        hash.try_into()
            .expect("RandomX backend returned an invalid hash length"),
        telemetry,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stage 1: Key-dependent Argon2d Cache initialization (256 MiB noise block).
    #[test]
    fn test_stage_01_cache_argon2d_derivation() {
        let cache = cache::Cache::new(b"test key 000");
        assert_eq!(
            cache.memory().len(),
            params::ARGON_MEMORY_KIB as usize * 1024
        );
        let first_word: [u8; 8] = cache.memory()[0..8].try_into().unwrap();
        assert_eq!(u64::from_le_bytes(first_word), 0x191e0e1d23c02186);
    }

    /// Stage 2: Superscalar Dataset register seeding from item indices (spec 7.3).
    #[test]
    fn test_stage_02_dataset_register_seeding() {
        let regs_0 = dataset::seed_registers(0);
        assert_eq!(regs_0[0], 0x5851f42d4c957f2d);

        let regs_1 = dataset::seed_registers(1);
        assert_eq!(regs_1[0], 0xb0a3e85a992afe5a);
    }

    /// Stage 3: Constant reciprocal calculation for division-free IMUL_RCP instruction.
    #[test]
    fn test_stage_03_reciprocal_division_helper() {
        assert_eq!(reciprocal::reciprocal(3), 12297829382473034410);
        assert_eq!(reciprocal::reciprocal(65537), 18446462603027742720);
        assert_eq!(reciprocal::reciprocal(4294967295), 9223372039002259456);
    }

    /// Stage 4a: AesGenerator1R pseudo-random expansion (1 AES round per lane).
    #[test]
    fn test_stage_04a_aes_generator_1r_state_expansion() {
        let mut state = [0u8; 64];
        let seed = hex::decode("6c19536eb2de31b6c0065f7f116e86f960d8af0c57210a6584c3237b9d064dc7")
            .unwrap();
        state[..seed.len()].copy_from_slice(&seed);
        aes_generator::fill_1r(&mut state);
        let expected = "fa89397dd6ca422513aeadba3f124b5540324c4ad4b6db434394307a17c833ab";
        assert_eq!(hex::encode(&state[..32]), expected);
    }

    /// Stage 4b: AesGenerator4R scratchpad initialization (4 AES rounds per lane).
    #[test]
    fn test_stage_04b_aes_generator_4r_scratchpad_fill() {
        let seed = [0x42u8; 64];
        let mut scratchpad = vec![0u8; 64 * 4];
        aes_generator::fill_4r(&seed, &mut scratchpad);
        assert_ne!(&scratchpad[..64], &[0u8; 64]);
    }

    /// Stage 5: VM initialization and end-to-end RandomX light-mode hash calculation.
    #[test]
    fn test_stage_05_vm_and_hash_execution_matches_reference_vector() {
        let key = b"test key 000";
        let input = b"This is a test";
        let hash = calculate_hash(key, input);
        assert_eq!(
            hex::encode(hash),
            "639183aae1bf4c9a35884cb46b09cad9175f04efd7684e7262a0ac1c2f0b4e3f"
        );
    }

    /// Stage 6: Different inputs or keys produce distinct 256-bit digests.
    #[test]
    fn test_stage_06_different_inputs_and_keys_produce_different_digests() {
        let key = b"test key 000";
        let input1 = b"This is a test";
        let input2 = b"Lorem ipsum dolor sit amet";
        let hash1 = calculate_hash(key, input1);
        let hash2 = calculate_hash(key, input2);
        assert_ne!(hash1, hash2);
        assert_eq!(
            hex::encode(hash2),
            "300a0adb47603dedb42228ccb2b211104f4da45af709cd7547cd049e9489c969"
        );
    }

    /// Stage 7: Telemetry captures duration, CPU time, CPU %, and memory across every stage.
    #[test]
    fn test_stage_07_telemetry_captures_cpu_and_memory_for_all_stages() {
        let key = b"test key 000";
        let input = b"This is a test";
        let (hash, telemetry) = calculate_hash_with_telemetry(key, input);
        assert_eq!(
            hex::encode(hash),
            "639183aae1bf4c9a35884cb46b09cad9175f04efd7684e7262a0ac1c2f0b4e3f"
        );

        let steps = telemetry.steps();
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].name, "cache_init");
        assert_eq!(steps[0].memory_bytes, 256 * 1024 * 1024);
        assert_eq!(steps[1].name, "vm_init");
        assert_eq!(steps[1].memory_bytes, 2 * 1024 * 1024);
        assert_eq!(steps[2].name, "hash_execute");
        assert_eq!(steps[2].memory_bytes, 258 * 1024 * 1024);

        assert!(telemetry.total().as_micros() > 0);
        assert_eq!(telemetry.peak_memory_bytes(), 258 * 1024 * 1024);
        assert!(telemetry.to_string().contains("cache_init"));
        assert!(telemetry.to_string().contains("hash_execute"));
        assert!(telemetry.to_string().contains("total / peak"));
    }
}
