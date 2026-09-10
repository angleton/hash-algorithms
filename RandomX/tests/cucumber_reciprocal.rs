//! BDD harness for the `reciprocal` step: `features/reciprocal/`.
//! Run with `cargo test --test cucumber_reciprocal`.

use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
pub struct ReciprocalWorld {
    divisor: u32,
    result: u64,
}

#[given(expr = "the divisor {int}")]
fn given_divisor(world: &mut ReciprocalWorld, divisor: u32) {
    world.divisor = divisor;
}

#[when("I compute its RandomX reciprocal")]
fn when_reciprocal(world: &mut ReciprocalWorld) {
    world.result = randomx_miner::reciprocal::reciprocal(world.divisor);
}

#[then(expr = "the reciprocal should equal {int}")]
fn then_reciprocal(world: &mut ReciprocalWorld, expected: u64) {
    assert_eq!(world.result, expected);
}

#[tokio::main]
async fn main() {
    ReciprocalWorld::run("features/reciprocal").await;
}
