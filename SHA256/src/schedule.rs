/// Expand the first sixteen message words into the full 64-word schedule.
pub fn expand_schedule(initial_words: [u32; 16]) -> [u32; 64] {
    let _ = initial_words;
    todo!("expand W[16..64] using SHA-256 sigma functions")
}