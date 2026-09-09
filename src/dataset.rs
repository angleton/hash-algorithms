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

/// Derive one 64-byte Dataset item on demand from the Cache (light mode).
pub fn get_item(_cache: &Cache, _item_number: u64) -> [u8; 64] {
    todo!("mix 8 cache reads through the 8 superscalar programs for this item index")
}
