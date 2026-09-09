//! RandomX algorithm parameters.
//!
//! These are the fixed constants defined by the official RandomX
//! specification (`doc/specs.md` in the reference C++ implementation).
//! Every one of these numbers has a direct effect on a single hash
//! computation, so they're written out here (rather than buried as magic
//! numbers) to make the pipeline in [`crate::calculate_hash`] easier to
//! follow.

/// Argon2d iterations used to build the Cache from the key.
pub const ARGON_ITERATIONS: u32 = 3;
/// Argon2d parallelism (lanes) used to build the Cache.
pub const ARGON_LANES: u32 = 1;
/// Argon2d memory size for the Cache, in 1 KiB blocks (256 MiB total).
pub const ARGON_MEMORY_KIB: u32 = 262_144;
/// Argon2d salt is the ASCII string "RandomX" followed by version byte 3.
pub const ARGON_SALT: &[u8] = b"RandomX\x03";

/// Size of a full Dataset item in bytes (one "mix" the VM reads per access).
pub const DATASET_ITEM_BYTES: usize = 64;
/// Number of Dataset items (light mode: derived on demand from the Cache).
pub const DATASET_BASE_SIZE: u64 = 2_147_483_648; // 2 GiB
pub const DATASET_EXTRA_SIZE: u64 = 33_554_368;

/// Number of chained programs executed per hash. Each program is generated
/// from the running state left behind by the previous one.
pub const PROGRAM_COUNT: usize = 8;
/// Number of VM instructions in one generated program.
pub const PROGRAM_SIZE: usize = 256;
/// Number of times each program's instruction loop is executed.
pub const PROGRAM_ITERATIONS: usize = 2048;

/// Total scratchpad size (L3): 2 MiB, addressed by the VM during execution.
pub const SCRATCHPAD_L3: usize = 2 * 1024 * 1024;
/// L2-sized addressing window within the scratchpad: 256 KiB.
pub const SCRATCHPAD_L2: usize = 256 * 1024;
/// L1-sized addressing window within the scratchpad: 16 KiB.
pub const SCRATCHPAD_L1: usize = 16 * 1024;

/// Number of 64-bit integer registers (r0..r7) in the VM register file.
pub const REGISTER_COUNT_INT: usize = 8;
/// Number of 128-bit floating point register groups (f, e, a each have 4).
pub const REGISTER_COUNT_FLT: usize = 4;

/// Size in bytes of the "register file" that seeds each program
/// (8 int registers + group A floating point registers).
pub const REGISTERS_SEED_BYTES: usize = 256;

/// Final output size of `calculate_hash`: a 256-bit Blake2b digest.
pub const HASH_SIZE: usize = 32;
