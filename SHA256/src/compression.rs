/// Compress one 512-bit block into the current SHA-256 chaining state.
pub fn compress_block(chaining_state: [u32; 8], block: [u8; 64]) -> [u32; 8] {
    let _ = chaining_state;
    let _ = block;
    todo!("expand the block schedule, run 64 rounds, and add back into the chaining state")
}