//! Cucumber BDD test harness for the full, end-to-end RandomX hash feature.
//!
//! Run with `cargo test --test cucumber`. Scenarios live in
//! `features/full_hash/randomx_hash.feature`; step definitions are below.
//! Per-step scenarios for individual pipeline stages live in their own
//! `features/<stage>/` folders with matching `tests/cucumber_<stage>.rs`
//! harnesses (see README.md).

use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
pub struct RandomXWorld {
    key: Vec<u8>,
    input: Vec<u8>,
    hash: [u8; randomx_miner::params::HASH_SIZE],
}

#[given(expr = "the key {string}")]
fn given_key(world: &mut RandomXWorld, key: String) {
    world.key = key.into_bytes();
}

#[given(expr = "the input {string}")]
fn given_input(world: &mut RandomXWorld, input: String) {
    world.input = input.into_bytes();
}

#[when("I calculate the RandomX hash")]
fn when_calculate(world: &mut RandomXWorld) {
    world.hash = randomx_miner::calculate_hash(&world.key, &world.input);
}

#[then(expr = "the resulting hash should equal {string}")]
fn then_hash_equals(world: &mut RandomXWorld, expected_hex: String) {
    let expected = hex::decode(expected_hex).expect("scenario hex string must be valid hex");
    assert_eq!(world.hash.as_slice(), expected.as_slice());
}

#[then(expr = "the resulting hash should not equal {string}")]
fn then_hash_not_equals(world: &mut RandomXWorld, unexpected_hex: String) {
    let unexpected = hex::decode(unexpected_hex).expect("scenario hex string must be valid hex");
    assert_ne!(world.hash.as_slice(), unexpected.as_slice());
}

#[tokio::main]
async fn main() {
    RandomXWorld::run("features/full_hash").await;
}
