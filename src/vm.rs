//! Stage 6: the **VM** — where a generated [`crate::program::Program`]
//! actually runs against the 2 MiB scratchpad.
//!
//! For each of the 8 chained programs, the VM:
//! 1. Loads the register file (8 integer + 4x3 float registers) from the
//!    program's entropy buffer.
//! 2. Executes the program's 256 instructions in a loop,
//!    `RANDOMX_PROGRAM_ITERATIONS` (2048) times, reading/writing the
//!    scratchpad at addresses derived from the integer registers.
//! 3. "Hashes and fills": mixes the register file into the scratchpad via
//!    AesGenerator4R, which both updates the scratchpad and produces the
//!    entropy buffer for the *next* program.
//!
//! After all 8 programs have run, the final register file is hashed with
//! Blake2b to produce the 32-byte RandomX output — that's the very last
//! step in [`crate::calculate_hash`].

use crate::program::Program;

/// The 8 integer + 12 floating point (f/e/a groups of 4) registers that
/// programs read and write.
#[derive(Debug, Default, Clone, Copy)]
pub struct RegisterFile {
    pub r: [u64; crate::params::REGISTER_COUNT_INT],
    // f, e, a float register groups intentionally omitted until the
    // floating point instructions are implemented.
}

/// A RandomX virtual machine bound to one Dataset/Cache and scratchpad.
pub struct Vm {
    scratchpad: Vec<u8>,
    registers: RegisterFile,
}

impl Vm {
    /// Create a VM with a freshly AES-filled scratchpad.
    pub fn new(scratchpad_seed: &[u8; 64]) -> Self {
        let _ = scratchpad_seed;
        todo!("allocate a 2 MiB scratchpad and fill it via AesGenerator4R")
    }

    /// Run one generated program for `RANDOMX_PROGRAM_ITERATIONS` loops,
    /// mutating the scratchpad and register file in place.
    pub fn execute(&mut self, _program: &Program) {
        todo!("interpret 256 instructions x 2048 iterations against the scratchpad")
    }

    /// Current register file, read after all 8 programs have executed.
    pub fn registers(&self) -> &RegisterFile {
        &self.registers
    }
}
