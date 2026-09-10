//! A from-scratch, educational Rust implementation of **RandomX**, the
//! proof-of-work hash algorithm used by Monero since November 2019.
//!
//! This crate is being built up one concept at a time. The very first goal
//! (this stage) is [`calculate_hash`]: computing a *single* RandomX hash
//! correctly, in "light mode" (256 MiB Cache, no precomputed Dataset), with
//! no attention paid yet to speed. Once a single hash matches the official
//! reference test vectors, later stages can focus on performance
//! (precomputed Dataset / "fast mode", SIMD, multithreading, JIT...).
//!
//! # Anatomy of one `calculate_hash(key, input)` call
//!
//! 1. **Cache** ([`cache`]) — derive 256 MiB of pseudo-random data from
//!    `key` using Argon2d. Expensive, but only needs to happen once per key
//!    (Monero changes the key roughly every 2048 blocks), not once per hash.
//! 2. **Superscalar programs** ([`superscalar`]) — derive 8 small integer
//!    programs from `key` used to expand Cache reads into full Dataset
//!    items on demand ("light mode").
//! 3. **Seed the first program** — hash `input` (and a bit of fixed
//!    context) to produce the initial 128-byte entropy buffer.
//! 4. **Run 8 chained programs** ([`program`], [`vm`]): for each of
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
pub mod program;
pub mod reciprocal;
pub mod superscalar;
pub mod telemetry;
pub mod vm;

use cache::Cache;
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
    let mut telemetry = HashTelemetry::new();
    let _cache = telemetry.time("cache_init", || Cache::new(key));
    let _ = input;
    todo!("run the 8 chained programs described in the module docs, then Blake2b the result")
}
