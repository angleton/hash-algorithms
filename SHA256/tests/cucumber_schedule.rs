//! BDD harness for SHA-256 message schedule expansion.

use cucumber::{World, given, then, when};
use sha256_educational::schedule::expand_schedule;

#[derive(Debug, World)]
pub struct ScheduleWorld {
    initial_words: [u32; 16],
    schedule: [u32; 64],
}

impl Default for ScheduleWorld {
    fn default() -> Self {
        Self {
            initial_words: [0; 16],
            schedule: [0; 64],
        }
    }
}

#[given(expr = "the first 16 schedule words {string}")]
fn given_initial_words(world: &mut ScheduleWorld, words_csv: String) {
    let words: Vec<u32> = words_csv
        .split(',')
        .map(|word| u32::from_str_radix(word.trim_start_matches("0x"), 16))
        .collect::<Result<_, _>>()
        .expect("scenario words must be valid hex");
    world.initial_words = words.try_into().expect("scenario must include 16 words");
}

#[when("I expand the SHA-256 message schedule")]
fn when_expand(world: &mut ScheduleWorld) {
    world.schedule = expand_schedule(world.initial_words);
}

#[then(expr = "schedule word {int} should equal {string}")]
fn then_schedule_word_equals(world: &mut ScheduleWorld, index: usize, expected_hex: String) {
    let expected = u32::from_str_radix(expected_hex.trim_start_matches("0x"), 16)
        .expect("scenario word must be valid hex");
    assert_eq!(world.schedule[index], expected);
}

#[tokio::main]
async fn main() {
    ScheduleWorld::cucumber()
        .with_default_cli()
        .run("features/schedule")
        .await;
}