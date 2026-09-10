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

pub const HASH_SIZE: usize = 32;

/// Compute the SHA-256 digest of `message`.
pub fn digest(message: &[u8]) -> [u8; HASH_SIZE] {
    let _ = message;
    todo!("run SHA-256 padding, block parsing, schedule expansion, compression, and digest serialization")
}