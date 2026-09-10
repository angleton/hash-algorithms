/// The eight SHA-256 compression working variables.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WorkingState {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    pub e: u32,
    pub f: u32,
    pub g: u32,
    pub h: u32,
}

/// Run one SHA-256 compression round.
pub fn compression_round(
    state: WorkingState,
    schedule_word: u32,
    round_constant: u32,
) -> WorkingState {
    let _ = state;
    let _ = schedule_word;
    let _ = round_constant;
    todo!("update the working variables for one SHA-256 compression round")
}