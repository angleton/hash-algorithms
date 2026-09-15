/// Parse one 512-bit block into sixteen big-endian 32-bit words.
pub fn first_block_words(block: &[u8; 64]) -> [u32; 16] {
    std::array::from_fn(|index| {
        u32::from_be_bytes(block[index * 4..index * 4 + 4].try_into().unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_02_block_parsing_big_endian_words() {
        let mut block = [0u8; 64];
        block[..4].copy_from_slice(&0x61626380u32.to_be_bytes());
        block[60..].copy_from_slice(&24u32.to_be_bytes());
        let words = first_block_words(&block);
        assert_eq!(words[0], 0x61626380);
        assert_eq!(words[15], 24);
    }

    #[test]
    fn test_stage_02_block_parsing_preserves_all_positions() {
        let expected = std::array::from_fn::<_, 16, _>(|index| index as u32 * 0x01010101);
        let mut block = [0u8; 64];
        for (index, word) in expected.iter().enumerate() {
            block[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        assert_eq!(first_block_words(&block), expected);
    }
}