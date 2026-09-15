//! BDD harness for SHA-256 block word parsing.

use cucumber::{World, given, then, when};
use sha256_educational::blocks::first_block_words;

#[derive(Debug, World)]
pub struct BlocksWorld {
    block: [u8; 64],
    words: [u32; 16],
}

impl Default for BlocksWorld {
    fn default() -> Self {
        Self {
            block: [0; 64],
            words: [0; 16],
        }
    }
}

#[given(expr = "the 512-bit block {string}")]
fn given_block(world: &mut BlocksWorld, block_hex: String) {
    let bytes = hex::decode(block_hex).expect("scenario block must be valid hex");
    world.block = bytes.try_into().expect("scenario block must be 64 bytes");
}

#[when("I parse the SHA-256 block words")]
fn when_parse_words(world: &mut BlocksWorld) {
    world.words = first_block_words(&world.block);
}

#[then(expr = "word {int} should equal {string}")]
fn then_word_equals(world: &mut BlocksWorld, index: usize, expected_hex: String) {
    let expected = u32::from_str_radix(expected_hex.trim_start_matches("0x"), 16)
        .expect("scenario word must be valid hex");
    assert_eq!(world.words[index], expected);
}

#[tokio::main]
async fn main() {
    BlocksWorld::cucumber()
        .with_default_cli()
        .run("features/blocks")
        .await;
}