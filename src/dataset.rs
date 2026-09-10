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
    let _ = (item_number, SEED_MUL0, SEED_ADD);
    todo!("r0 = (item_number + 1).wrapping_mul(SEED_MUL0); r[1..=7] = r0 ^ SEED_ADD[..]")
}

/// Derive one 64-byte Dataset item on demand from the Cache (light mode).
pub fn get_item(_cache: &Cache, _item_number: u64) -> [u8; 64] {
    todo!("seed_registers, then mix in 8 cache reads through the 8 superscalar programs")
}
