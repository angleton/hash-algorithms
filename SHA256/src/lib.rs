//! A from-scratch, educational SHA-256 scaffold.
//!
//! The crate is organized around the standard SHA-256 pipeline, with one BDD
//! feature per stage. Implementations are intentionally pending so the tests
//! describe the target behavior before any code is written.

pub mod blocks;
pub mod compression;
pub mod padding;
pub mod round;
pub mod schedule;
pub mod telemetry;

pub const HASH_SIZE: usize = 32;

/// Compute the SHA-256 digest of `message`.
pub fn digest(message: &[u8]) -> [u8; HASH_SIZE] {
    digest_with_telemetry(message).0
}

pub fn digest_with_telemetry(message: &[u8]) -> ([u8; HASH_SIZE], telemetry::HashTelemetry) {
    let mut telemetry = telemetry::HashTelemetry::new();
    let padded = telemetry.time("stage_01_padding", message.len() + 64, || {
        padding::pad_message(message)
    });
    let mut state = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    for (index, chunk) in padded.chunks_exact(64).enumerate() {
        let block: [u8; 64] = telemetry.time("stage_02_block_parsing", 64, || {
            chunk.try_into().expect("padded chunks are 64 bytes")
        });
        let words = telemetry.time("stage_03_schedule_expansion", 64, || {
            blocks::first_block_words(&block)
        });
        let schedule = telemetry.time("stage_04_schedule_ready", 256, || {
            schedule::expand_schedule(words)
        });
        state = telemetry.time("stage_05_compression", 64 + 256, || {
            compression::compress_schedule(state, schedule)
        });
        let _ = index;
    }
    let digest = telemetry.time("stage_06_digest_serialization", HASH_SIZE, || {
        let mut output = [0u8; HASH_SIZE];
        for (chunk, word) in output.chunks_exact_mut(4).zip(state) {
            chunk.copy_from_slice(&word.to_be_bytes());
        }
        output
    });
    (digest, telemetry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_01_to_06_empty_message_digest() {
        assert_eq!(
            hex::encode(digest(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_stage_01_to_06_abc_message_digest() {
        assert_eq!(
            hex::encode(digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_stage_06_digest_changes_when_message_changes() {
        assert_ne!(digest(b"abc"), digest(b"abd"));
    }

    #[test]
    fn test_stage_07_telemetry_reports_ordered_pipeline_stages() {
        let (hash, telemetry) = digest_with_telemetry(b"abc");
        assert_eq!(hash, digest(b"abc"));
        let names: Vec<_> = telemetry.steps().iter().map(|step| step.name).collect();
        assert_eq!(
            names,
            [
                "stage_01_padding",
                "stage_02_block_parsing",
                "stage_03_schedule_expansion",
                "stage_04_schedule_ready",
                "stage_05_compression",
                "stage_06_digest_serialization",
            ]
        );
        assert!(telemetry.total().as_micros() > 0);
        assert_eq!(telemetry.peak_memory_bytes(), 320);
    }
}