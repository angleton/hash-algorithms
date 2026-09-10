//! `reciprocal(divisor)`: the helper behind the `IMUL_RCP` instruction.
//!
//! RandomX programs never contain a division instruction (division is slow
//! and its timing can depend on the operands, which the algorithm wants to
//! avoid). Instead, `IMUL_RCP` multiplies by a precomputed 64-bit
//! reciprocal of a fixed, program-embedded divisor. `reciprocal` is the
//! function that computes that constant once (when a Cache's superscalar
//! programs are generated), so it belongs conceptually to the Cache stage,
//! but it's pure integer math with no dependency on anything else — a good
//! first, fully isolated thing to implement and test.

/// Compute the 64-bit reciprocal used by `IMUL_RCP` for a given 32-bit
/// divisor. `divisor` is never 0 or a power of 2 (RandomX program
/// generation guarantees this), so there's exactly one well-defined answer
/// for every valid input.
pub fn reciprocal(divisor: u32) -> u64 {
    let _ = divisor;
    todo!("compute the 2^x / divisor style reciprocal used by IMUL_RCP")
}
