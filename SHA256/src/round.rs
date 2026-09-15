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
    let ch = (state.e & state.f) ^ ((!state.e) & state.g);
    let maj = (state.a & state.b) ^ (state.a & state.c) ^ (state.b & state.c);
    let big_sigma1 = state.e.rotate_right(6)
        ^ state.e.rotate_right(11)
        ^ state.e.rotate_right(25);
    let big_sigma0 = state.a.rotate_right(2)
        ^ state.a.rotate_right(13)
        ^ state.a.rotate_right(22);
    let temp1 = state
        .h
        .wrapping_add(big_sigma1)
        .wrapping_add(ch)
        .wrapping_add(round_constant)
        .wrapping_add(schedule_word);
    let temp2 = big_sigma0.wrapping_add(maj);
    WorkingState {
        a: temp1.wrapping_add(temp2),
        b: state.a,
        c: state.b,
        d: state.c,
        e: state.d.wrapping_add(temp1),
        f: state.e,
        g: state.f,
        h: state.g,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_04_first_abc_compression_round() {
        let state = WorkingState {
            a: 0x6a09e667, b: 0xbb67ae85, c: 0x3c6ef372, d: 0xa54ff53a,
            e: 0x510e527f, f: 0x9b05688c, g: 0x1f83d9ab, h: 0x5be0cd19,
        };
        assert_eq!(
            compression_round(state, 0x61626380, 0x428a2f98),
            WorkingState { a: 0x5d6aebcd, b: 0x6a09e667, c: 0xbb67ae85, d: 0x3c6ef372,
                e: 0xfa2a4622, f: 0x510e527f, g: 0x9b05688c, h: 0x1f83d9ab }
        );
    }
}