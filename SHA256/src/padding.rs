/// Pad a message according to SHA-256 preprocessing rules.
pub fn pad_message(message: &[u8]) -> Vec<u8> {
    let _ = message;
    todo!("append the 1 bit, zero padding, and 64-bit big-endian message length")
}