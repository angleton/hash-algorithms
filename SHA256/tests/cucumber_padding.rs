//! BDD harness for SHA-256 message padding.

use cucumber::{World, given, then, when};
use sha256_educational::padding::pad_message;

#[derive(Debug, Default, World)]
pub struct PaddingWorld {
    message: Vec<u8>,
    padded: Vec<u8>,
}

#[given(expr = "the message {string}")]
fn given_message(world: &mut PaddingWorld, message: String) {
    world.message = message.into_bytes();
}

#[when("I pad the SHA-256 message")]
fn when_pad(world: &mut PaddingWorld) {
    world.padded = pad_message(&world.message);
}

#[then(expr = "the padded message should equal {string}")]
fn then_padded_equals(world: &mut PaddingWorld, expected_hex: String) {
    let expected = hex::decode(expected_hex).expect("scenario hex string must be valid hex");
    assert_eq!(world.padded, expected);
}

#[tokio::main]
async fn main() {
    PaddingWorld::cucumber()
        .with_default_cli()
        .run("features/padding")
        .await;
}