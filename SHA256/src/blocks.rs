/// Parse one 512-bit block into sixteen big-endian 32-bit words.
pub fn first_block_words(block: &[u8; 64]) -> [u32; 16] {
    let _ = block;
    todo!("parse the 64-byte block as sixteen big-endian u32 words")
}