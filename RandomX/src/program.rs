//! Stage 5: **program generation**.
//!
//! Each of the 8 chained programs is generated from a 128-byte "entropy"
//! buffer produced by [`crate::aes_generator`]. The entropy buffer supplies:
//! - the initial values of the 8 integer registers `r0..r7` (used to hold
//!   scratchpad addresses `mx`/`ma`, and floating point `a0..a3` registers),
//! - a "program configuration" (rounding mode, register groups eligible for
//!   the final float mask), and
//! - the byte stream that [`decode`] turns into 256 8-byte VM instructions.
//!
//! This is where RandomX gets its name: every hash executes a *different*,
//! effectively-random sequence of integer/float/branch instructions.

/// One decoded VM instruction: opcode + 3 operand bytes, matching the
/// 8-byte-per-instruction layout used by the reference implementation.
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub dst: u8,
    pub src: u8,
    pub mod_: u8,
    pub imm32: u32,
}

/// A fully decoded RandomX program: 256 instructions plus the register/
/// rounding configuration derived from the entropy buffer.
pub struct Program {
    pub instructions: [Instruction; crate::params::PROGRAM_SIZE],
}

/// Turn a 128-byte entropy buffer (produced by AesGenerator1R from the
/// previous program's output, or from the initial seed for program 0) into
/// a decoded [`Program`].
pub fn generate(_entropy: &[u8; 128]) -> Program {
    todo!("decode 256 8-byte instructions from the entropy buffer")
}
