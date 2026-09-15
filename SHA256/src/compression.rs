use crate::blocks::first_block_words;
use crate::round::{compression_round, WorkingState};
use crate::schedule::expand_schedule;

#[cfg(test)]
const INITIAL_STATE: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Compress one 512-bit block into the current SHA-256 chaining state.
pub fn compress_block(chaining_state: [u32; 8], block: [u8; 64]) -> [u32; 8] {
    compress_schedule(chaining_state, expand_schedule(first_block_words(&block)))
}

pub(crate) fn compress_schedule(chaining_state: [u32; 8], schedule: [u32; 64]) -> [u32; 8] {
    let mut state = WorkingState {
        a: chaining_state[0], b: chaining_state[1], c: chaining_state[2], d: chaining_state[3],
        e: chaining_state[4], f: chaining_state[5], g: chaining_state[6], h: chaining_state[7],
    };
    for (&word, &constant) in schedule.iter().zip(ROUND_CONSTANTS.iter()) {
        state = compression_round(state, word, constant);
    }
    [
        chaining_state[0].wrapping_add(state.a), chaining_state[1].wrapping_add(state.b),
        chaining_state[2].wrapping_add(state.c), chaining_state[3].wrapping_add(state.d),
        chaining_state[4].wrapping_add(state.e), chaining_state[5].wrapping_add(state.f),
        chaining_state[6].wrapping_add(state.g), chaining_state[7].wrapping_add(state.h),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_05_compress_abc_block_reference_state() {
        let mut block = [0u8; 64];
        block[..3].copy_from_slice(b"abc");
        block[3] = 0x80;
        block[63] = 24;
        assert_eq!(
            compress_block(INITIAL_STATE, block),
            [0xba7816bf, 0x8f01cfea, 0x414140de, 0x5dae2223,
             0xb00361a3, 0x96177a9c, 0xb410ff61, 0xf20015ad]
        );
    }
}