//! BDD harness for the `cache` step: `features/cache/`.
//! Run with `cargo test --test cucumber_cache`.

use cucumber::{World, given, then, when};
use randomx_miner::cache::Cache;

#[derive(Debug, Default, World)]
pub struct CacheWorld {
    key: Vec<u8>,
    cache: Option<Cache>,
}

#[given(expr = "the key {string}")]
fn given_key(world: &mut CacheWorld, key: String) {
    world.key = key.into_bytes();
}

#[when("I build the RandomX cache")]
fn when_build_cache(world: &mut CacheWorld) {
    world.cache = Some(Cache::new(&world.key));
}

#[then(expr = "8-byte word {int} of the cache should equal {string}")]
fn then_word_equals(world: &mut CacheWorld, index: usize, expected_hex: String) {
    let expected = u64::from_str_radix(expected_hex.trim_start_matches("0x"), 16)
        .expect("scenario hex value must be valid hex");
    let memory = world.cache.as_ref().expect("cache must be built first").memory();
    let word_bytes: [u8; 8] = memory[index * 8..index * 8 + 8]
        .try_into()
        .expect("8 bytes");
    assert_eq!(u64::from_le_bytes(word_bytes), expected);
}

#[tokio::test]
async fn cucumber_cache() {
    CacheWorld::run("features/cache").await;
}
