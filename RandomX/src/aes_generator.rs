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

use aes::Block;
use aes::hazmat::{cipher_round, equiv_inv_cipher_round};

const AES_GEN_1R_KEYS: [[u32; 4]; 4] = [
    [0xb4f44917, 0xdbb5552b, 0x62716609, 0x6daca553],
    [0x0da1dc4e, 0x1725d378, 0x846a710d, 0x6d7caf07],
    [0x3e20e345, 0xf4c0794f, 0x9f947ec6, 0x3f1262f1],
    [0x49169154, 0x16314c88, 0xb1ba317c, 0x6aef8135],
];

const AES_GEN_4R_KEYS: [[u32; 4]; 8] = [
    [0x99e5d23f, 0x2f546d2b, 0xd1833ddb, 0x6421aadd],
    [0xa5dfcde5, 0x06f79d53, 0xb6913f55, 0xb20e3450],
    [0x171c02bf, 0x0aa4679f, 0x515e7baf, 0x5c3ed904],
    [0xd8ded291, 0xcd673785, 0xe78f5d08, 0x85623763],
    [0x229effb4, 0x3d518b6d, 0xe3d6a7a6, 0xb5826f73],
    [0xb272b7d2, 0xe9024d4e, 0x9c10b3d9, 0xc7566bf3],
    [0xf63befa7, 0x2ba9660a, 0xf765a38b, 0xf273c9e7],
    [0xc0b0762d, 0x0c06d1fd, 0x915839de, 0x7a7cd609],
];

fn round_key(words_high_to_low: [u32; 4]) -> Block {
    let mut bytes = [0u8; 16];
    for (chunk, word) in bytes.chunks_exact_mut(4).zip(words_high_to_low.into_iter().rev()) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    Block::from(bytes)
}

fn load_lanes(state: &[u8; 64]) -> [Block; 4] {
    std::array::from_fn(|lane| {
        let bytes: [u8; 16] = state[lane * 16..(lane + 1) * 16]
            .try_into()
            .expect("lane is exactly 16 bytes");
        Block::from(bytes)
    })
}

fn store_lanes(lanes: &[Block; 4], output: &mut [u8]) {
    for (destination, lane) in output.chunks_exact_mut(16).zip(lanes) {
        destination.copy_from_slice(lane);
    }
}

/// Fill `state` in place with pseudo-random bytes, applying one AES round
/// per 16-byte lane per output block (`AesGenerator1R`). Matches the
/// reference `fillAes1Rx4(state, sizeof(state), state)` call, where the
/// same buffer is both the seed and the output.
pub fn fill_1r(state: &mut [u8; 64]) {
    let keys = AES_GEN_1R_KEYS.map(round_key);
    let mut lanes = load_lanes(state);
    equiv_inv_cipher_round(&mut lanes[0], &keys[0]);
    cipher_round(&mut lanes[1], &keys[1]);
    equiv_inv_cipher_round(&mut lanes[2], &keys[2]);
    cipher_round(&mut lanes[3], &keys[3]);
    store_lanes(&lanes, state);
}

/// Fill `output` with pseudo-random bytes derived from `seed`, applying four
/// AES rounds per state block per step (`AesGenerator4R`). Also used in
/// "hash and fill" mode to simultaneously consume scratchpad contents and
/// produce the next seed.
pub fn fill_4r(seed: &[u8; 64], output: &mut [u8]) {
    assert_eq!(output.len() % 64, 0, "AES4R output must be a multiple of 64 bytes");
    let keys = AES_GEN_4R_KEYS.map(round_key);
    let mut lanes = load_lanes(seed);

    for block in output.chunks_exact_mut(64) {
        for round in 0..4 {
            equiv_inv_cipher_round(&mut lanes[0], &keys[round]);
            cipher_round(&mut lanes[1], &keys[round]);
            equiv_inv_cipher_round(&mut lanes[2], &keys[round + 4]);
            cipher_round(&mut lanes[3], &keys[round + 4]);
        }
        store_lanes(&lanes, block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage4a_aes_generator_1r_reference_vector() {
        let mut state = [0u8; 64];
        let seed = hex::decode("6c19536eb2de31b6c0065f7f116e86f960d8af0c57210a6584c3237b9d064dc7")
            .unwrap();
        state[..seed.len()].copy_from_slice(&seed);
        fill_1r(&mut state);
        let expected = "fa89397dd6ca422513aeadba3f124b5540324c4ad4b6db434394307a17c833ab";
        assert_eq!(hex::encode(&state[..32]), expected);
    }

    #[test]
    fn test_stage4b_aes_generator_4r_fills_multiples_of_64_bytes() {
        let seed = [0x5au8; 64];
        let mut output = vec![0u8; 128];
        fill_4r(&seed, &mut output);
        assert_ne!(&output[..64], &[0u8; 64]);
        assert_ne!(&output[64..], &[0u8; 64]);
    }
}
