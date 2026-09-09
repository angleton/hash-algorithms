//! Stage 4: **AesGenerator**, RandomX's AES-based pseudo-random byte fill.
//!
//! RandomX uses fixed-key AES round functions (no key schedule, just
//! repeated `aesenc`/`aesdec`) as a fast, hardware-accelerated way to fill
//! large buffers with pseudo-random bytes from a small seed. It's used
//! twice in a single hash:
//!
//! 1. **AesGenerator1R** fills the register-seed buffer that configures
//!    each of the 8 chained programs (`RANDOMX_PROGRAM_COUNT`).
//! 2. **AesGenerator4R** fills the 2 MiB scratchpad at the start of a hash,
//!    and mixes it again ("hashAndFill") after every program to produce the
//!    seed for the *next* program plus the final register fingerprint.
//!
//! The "1R"/"4R" naming refers to how many AES rounds are applied per
//! 16-byte block per generator step.

/// Fill `output` with pseudo-random bytes derived from `seed`, applying one
/// AES round per state block per step (`AesGenerator1R`).
pub fn fill_1r(_seed: &[u8; 64], output: &mut [u8]) {
    let _ = output;
    todo!("apply fixed-key AES encryption/decryption rounds to expand the seed")
}

/// Fill `output` with pseudo-random bytes derived from `seed`, applying four
/// AES rounds per state block per step (`AesGenerator4R`). Also used in
/// "hash and fill" mode to simultaneously consume scratchpad contents and
/// produce the next seed.
pub fn fill_4r(_seed: &[u8; 64], output: &mut [u8]) {
    let _ = output;
    todo!("apply fixed-key AES encryption/decryption rounds to expand the seed")
}
