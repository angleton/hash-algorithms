//! BDD harness for SHA-256 block compression.

use cucumber::{World, given, then, when};
use sha256::compression::compress_block;

#[derive(Debug, World)]
pub struct CompressionWorld {
    chaining_state: [u32; 8],
    block: [u8; 64],
}

impl Default for CompressionWorld {
    fn default() -> Self {
        Self {
            chaining_state: [0; 8],
            block: [0; 64],
        }
    }
}

fn parse_words(words_csv: &str) -> [u32; 8] {
    let words: Vec<u32> = words_csv
        .split(',')
        .map(|word| u32::from_str_radix(word.trim_start_matches("0x"), 16))
        .collect::<Result<_, _>>()
        .expect("scenario words must be valid hex");
    words.try_into().expect("scenario must include 8 words")
}

#[given(expr = "the chaining state {string}")]
fn given_chaining_state(world: &mut CompressionWorld, words_csv: String) {
    world.chaining_state = parse_words(&words_csv);
}

#[given(expr = "the 512-bit block {string}")]
fn given_block(world: &mut CompressionWorld, block_hex: String) {
    let bytes = hex::decode(block_hex).expect("scenario block must be valid hex");
    world.block = bytes.try_into().expect("scenario block must be 64 bytes");
}

#[when("I compress the SHA-256 block")]
fn when_compress(world: &mut CompressionWorld) {
    world.chaining_state = compress_block(world.chaining_state, world.block);
}

#[then(expr = "the chaining state should equal {string}")]
fn then_chaining_state_equals(world: &mut CompressionWorld, expected_csv: String) {
    assert_eq!(world.chaining_state, parse_words(&expected_csv));
}

#[tokio::main]
async fn main() {
    CompressionWorld::cucumber()
        .with_default_cli()
        .run("features/compression")
        .await;
}