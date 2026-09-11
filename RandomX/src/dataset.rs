//! Stage 3: the **Dataset**, accessed one item at a time.
//!
//! Conceptually the Dataset is 2 GiB+ of pseudo-random data derived from the
//! Cache. In "fast mode" it's fully precomputed once and reused for many
//! hashes (fast, but needs ~2080 MiB RAM). In "light mode" each item is
//! derived on demand straight from the [`crate::cache`] using the
//! [`crate::superscalar`] programs (slow, but needs only ~256 MiB RAM).
//!
//! Both modes are required to produce byte-for-byte identical results —
//! that's what makes light mode useful for verifying someone else's hash
//! cheaply. We start with light mode since it's simpler to get right first.

use crate::cache::Cache;

/// Multiplier and XOR constants from RandomX spec section 7.3, used to
/// seed a Dataset item's 8 integer registers from only the item number,
/// before any Cache reads happen. Verified identical across the reference
/// implementation's `dataset.cpp`, `doc/specs.md`, and `superscalar-init.cpp`.
const SEED_MUL0: u64 = 6_364_136_223_846_793_005;
const SEED_ADD: [u64; 7] = [
    9_298_411_001_130_361_340,
    12_065_312_585_734_608_966,
    9_306_329_213_124_626_780,
    5_281_919_268_842_080_866,
    10_536_153_434_571_861_004,
    3_398_623_926_847_679_864,
    9_549_104_520_008_361_294,
];

/// Step 1 of Dataset block generation (spec 7.3): seed the 8 integer
/// registers purely from the item number. Pure arithmetic, no Cache access
/// — the simplest possible independently-testable piece of Dataset
/// generation.
pub fn seed_registers(item_number: u64) -> [u64; 8] {
    let r0 = (item_number.wrapping_add(1)).wrapping_mul(SEED_MUL0);
    let mut registers = [0u64; 8];
    registers[0] = r0;
    for (register, add) in registers[1..].iter_mut().zip(SEED_ADD) {
        *register = r0 ^ add;
    }
    registers
}

/// Derive one 64-byte Dataset item on demand from the Cache (light mode).
pub fn get_item(_cache: &Cache, _item_number: u64) -> [u8; 64] {
    todo!("seed_registers, then mix in 8 cache reads through the 8 superscalar programs")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independently computed from the documented formula/constants, cross-checked
    // against dataset.cpp, doc/specs.md, and superscalar-init.cpp.
    #[test]
    fn matches_reference_vectors() {
        let cases: [(u64, [u64; 8]); 3] = [
            (
                0,
                [
                    0x5851f42d4c957f2d,
                    0xd95b63a71560ded1,
                    0xff216df27457a76b,
                    0xd9774d31f3b73671,
                    0x111cd1ba5b0af54f,
                    0xca661b94823f9321,
                    0x777ba25920735255,
                    0xdcd4cfdafab99a63,
                ],
            ),
            (
                1,
                [
                    0xb0a3e85a992afe5a,
                    0x31a97fd0c0df5fa6,
                    0x17d37185a1e8261c,
                    0x318551462608b706,
                    0xf9eecdcd8eb57438,
                    0x229407e357801256,
                    0x9f89be2ef5ccd322,
                    0x3426d3ad2f061b14,
                ],
            ),
            (
                1_000_000,
                [
                    0xdc6a2a017061e46d,
                    0x5d60bd8b29944591,
                    0x7b1ab3de48a33c2b,
                    0x5d4c931dcf43ad31,
                    0x95270f9667fe6e0f,
                    0x4e5dc5b8becb0861,
                    0xf3407c751c87c915,
                    0x58ef11f6c64d0123,
                ],
            ),
        ];

        for (item_number, expected) in cases {
            assert_eq!(seed_registers(item_number), expected, "item_number {item_number}");
        }
    }
}
