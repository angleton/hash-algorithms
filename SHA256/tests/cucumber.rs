//! BDD harness for the full, end-to-end SHA-256 hash feature.

use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
pub struct Sha256World {
    message: Vec<u8>,
    digest: [u8; sha256_educational::HASH_SIZE],
}

#[given(expr = "the message {string}")]
fn given_message(world: &mut Sha256World, message: String) {
    world.message = message.into_bytes();
}

#[when("I calculate the SHA-256 hash")]
fn when_calculate(world: &mut Sha256World) {
    world.digest = sha256_educational::digest(&world.message);
}

#[then(expr = "the resulting hash should equal {string}")]
fn then_hash_equals(world: &mut Sha256World, expected_hex: String) {
    let expected = hex::decode(expected_hex).expect("scenario hex string must be valid hex");
    assert_eq!(world.digest.as_slice(), expected.as_slice());
}

#[tokio::main]
async fn main() {
    Sha256World::run("features/full_hash").await;
}