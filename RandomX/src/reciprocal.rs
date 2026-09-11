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
    let divisor = divisor as u64;
    const P2EXP63: u64 = 1u64 << 63;

    let mut quotient = P2EXP63 / divisor;
    let mut remainder = P2EXP63 % divisor;

    let bit_length = 64 - divisor.leading_zeros();
    for _ in 0..bit_length {
        if remainder >= divisor - remainder {
            quotient = quotient * 2 + 1;
            remainder = remainder * 2 - divisor;
        } else {
            quotient *= 2;
            remainder *= 2;
        }
    }

    quotient
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference values from src/tests/tests.cpp ("randomx_reciprocal" test).
    #[test]
    fn matches_reference_vectors() {
        let cases: [(u32, u64); 7] = [
            (3, 12297829382473034410),
            (13, 11351842506898185609),
            (33, 17887751829051686415),
            (65537, 18446462603027742720),
            (15000001, 10316166306300415204),
            (3845182035, 10302264209224146340),
            (4294967295, 9223372039002259456),
        ];

        for (divisor, expected) in cases {
            assert_eq!(reciprocal(divisor), expected, "divisor {divisor}");
        }
    }
}
