//! BDD harness for one SHA-256 compression round.

use cucumber::{World, given, then, when};
use sha256_educational::round::{WorkingState, compression_round};

#[derive(Debug, Default, World)]
pub struct RoundWorld {
    state: WorkingState,
    schedule_word: u32,
    round_constant: u32,
}

fn parse_state(words_csv: &str) -> WorkingState {
    let words: Vec<u32> = words_csv
        .split(',')
        .map(|word| u32::from_str_radix(word.trim_start_matches("0x"), 16))
        .collect::<Result<_, _>>()
        .expect("scenario state must be valid hex");
    let words: [u32; 8] = words.try_into().expect("scenario state must have 8 words");
    WorkingState {
        a: words[0],
        b: words[1],
        c: words[2],
        d: words[3],
        e: words[4],
        f: words[5],
        g: words[6],
        h: words[7],
    }
}

#[given(expr = "the working state {string}")]
fn given_state(world: &mut RoundWorld, words_csv: String) {
    world.state = parse_state(&words_csv);
}

#[given(expr = "the schedule word {string}")]
fn given_schedule_word(world: &mut RoundWorld, word_hex: String) {
    world.schedule_word = u32::from_str_radix(word_hex.trim_start_matches("0x"), 16)
        .expect("scenario word must be valid hex");
}

#[given(expr = "the round constant {string}")]
fn given_round_constant(world: &mut RoundWorld, constant_hex: String) {
    world.round_constant = u32::from_str_radix(constant_hex.trim_start_matches("0x"), 16)
        .expect("scenario constant must be valid hex");
}

#[when("I run one SHA-256 compression round")]
fn when_round(world: &mut RoundWorld) {
    world.state = compression_round(world.state, world.schedule_word, world.round_constant);
}

#[then(expr = "the working state should equal {string}")]
fn then_state_equals(world: &mut RoundWorld, expected_csv: String) {
    assert_eq!(world.state, parse_state(&expected_csv));
}

#[tokio::main]
async fn main() {
    RoundWorld::run("features/round").await;
}