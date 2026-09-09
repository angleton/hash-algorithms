//! Stage 2: **SuperscalarHash** programs, used to expand the [`crate::cache`]
//! into individual Dataset items without needing the full 2 GiB Dataset in
//! memory.
//!
//! RandomX generates `RANDOMX_CACHE_ACCESSES` (8) small integer-only
//! "superscalar" programs from the key using [`crate::blake2_generator`].
//! To compute Dataset item `i`, the Cache is read at 8 pseudo-random
//! locations (derived from `i`) and each value is mixed through one of the
//! 8 superscalar programs. This is the expensive part of "light mode" — it
//! trades the 2 GiB Dataset for CPU work, executed for every memory access
//! the main VM program makes.
//!
//! For a first pass we only need this to work correctly, not quickly.

/// A single generated superscalar program (a fixed sequence of simple
/// integer instructions: IMUL_R, IADD_RS, ISUB_R, IXOR_R, IROR_C, IMUL_RCP, ...).
pub struct SuperscalarProgram {
    // TODO: Vec<Instruction> once the instruction set is modeled.
}

/// Generate the 8 superscalar programs used to expand the Cache into
/// Dataset items ("SuperscalarHash generator" in the reference tests).
pub fn generate_programs(_cache_seed_key: &[u8]) -> [SuperscalarProgram; 8] {
    todo!("derive 8 SuperscalarPrograms from a Blake2Generator seeded with the key")
}
