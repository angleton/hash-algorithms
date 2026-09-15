/// Expand the first sixteen message words into the full 64-word schedule.
pub fn expand_schedule(initial_words: [u32; 16]) -> [u32; 64] {
    let mut schedule = [0u32; 64];
    schedule[..16].copy_from_slice(&initial_words);
    for index in 16..64 {
        let s0 = schedule[index - 15].rotate_right(7)
            ^ schedule[index - 15].rotate_right(18)
            ^ (schedule[index - 15] >> 3);
        let s1 = schedule[index - 2].rotate_right(17)
            ^ schedule[index - 2].rotate_right(19)
            ^ (schedule[index - 2] >> 10);
        schedule[index] = schedule[index - 16]
            .wrapping_add(s0)
            .wrapping_add(schedule[index - 7])
            .wrapping_add(s1);
    }
    schedule
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_03_schedule_keeps_initial_words() {
        let initial = [0x61626380, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24];
        let schedule = expand_schedule(initial);
        assert_eq!(&schedule[..16], &initial);
    }

    #[test]
    fn test_stage_03_schedule_abc_reference_words() {
        let initial = [0x61626380, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24];
        let schedule = expand_schedule(initial);
        assert_eq!(schedule[16], 0x61626380);
        assert_eq!(schedule[17], 0x000f0000);
        assert_eq!(schedule[63], 0x12b1edeb);
    }
}