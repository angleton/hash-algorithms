/// Pad a message according to SHA-256 preprocessing rules.
pub fn pad_message(message: &[u8]) -> Vec<u8> {
    let bit_length = (message.len() as u64).wrapping_mul(8);
    let padded_len = ((message.len() + 9 + 63) / 64) * 64;
    let mut padded = Vec::with_capacity(padded_len);
    padded.extend_from_slice(message);
    padded.push(0x80);
    padded.resize(padded_len - 8, 0);
    padded.extend_from_slice(&bit_length.to_be_bytes());
    padded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_01_padding_empty_message() {
        let padded = pad_message(b"");
        assert_eq!(padded.len(), 64);
        assert_eq!(padded[0], 0x80);
        assert_eq!(&padded[56..], &[0u8; 8]);
    }

    #[test]
    fn test_stage_01_padding_abc_message() {
        let padded = pad_message(b"abc");
        assert_eq!(padded.len(), 64);
        assert_eq!(&padded[..4], &[b'a', b'b', b'c', 0x80]);
        assert_eq!(&padded[56..], &24u64.to_be_bytes());
    }

    #[test]
    fn test_stage_01_padding_56_bytes_uses_two_blocks() {
        let padded = pad_message(&[0xa5; 56]);
        assert_eq!(padded.len(), 128);
        assert_eq!(padded[55], 0xa5);
        assert_eq!(padded[56], 0x80);
        assert_eq!(&padded[57..64], &[0u8; 7]);
        assert_eq!(&padded[120..], &448u64.to_be_bytes());
    }
}