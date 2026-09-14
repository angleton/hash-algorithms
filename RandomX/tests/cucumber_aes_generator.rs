//! BDD harness for the `aes_generator` step: `features/aes_generator/`.
//! Run with `cargo test --test cucumber_aes_generator`.

use cucumber::{World, given, then, when};

#[derive(Debug, World)]
pub struct AesGeneratorWorld {
    state: [u8; 64],
}

impl Default for AesGeneratorWorld {
    fn default() -> Self {
        Self { state: [0u8; 64] }
    }
}

#[given(expr = "the AES generator state {string}")]
fn given_state(world: &mut AesGeneratorWorld, hex_str: String) {
    let bytes = hex::decode(&hex_str).expect("scenario hex value must be valid hex");
    world.state = [0u8; 64];
    world.state[..bytes.len()].copy_from_slice(&bytes);
}

#[when("I run AesGenerator1R for one block")]
fn when_fill(world: &mut AesGeneratorWorld) {
    randomx_miner::aes_generator::fill_1r(&mut world.state);
}

#[then(expr = "the first 32 bytes of the resulting state should equal {string}")]
fn then_state(world: &mut AesGeneratorWorld, expected_hex: String) {
    let expected = hex::decode(&expected_hex).expect("scenario hex value must be valid hex");
    assert_eq!(&world.state[..expected.len()], expected.as_slice());
}

#[tokio::test]
async fn cucumber_aes_generator() {
    AesGeneratorWorld::cucumber()
        .with_default_cli()
        .run("features/aes_generator")
        .await;
}
