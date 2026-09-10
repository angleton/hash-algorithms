//! Stage 1 of a RandomX hash: the **Cache**.
//!
//! The Cache is a large (256 MiB) memory area derived from the RandomX key
//! `K` using the Argon2d password hashing function. It changes only when the
//! key changes (in Monero, roughly every 2048 blocks) — it is *not*
//! recomputed for every hash.
//!
//! In "light mode" (what this project implements first, since it needs only
//! 256 MiB instead of the 2 GiB+ "fast mode" Dataset) every Dataset item
//! used during VM execution is derived on the fly from the Cache via
//! [`crate::superscalar`] programs. That keeps memory usage low at the cost
//! of speed, which is fine for understanding/verifying a hash before we
//! worry about making mining fast.
//!
//! Argon2d parameters (see [`crate::params`]):
//! - password = the key `K`
//! - salt = `"RandomX\x03"`
//! - iterations = 3, lanes = 1, memory = 262144 KiB (256 MiB)

use crate::params;

/// The RandomX Cache: 256 MiB derived from the key via Argon2d.
#[derive(Debug)]
pub struct Cache {
    memory: Vec<u8>,
}

impl Cache {
    /// Build the Cache from a RandomX key by running Argon2d over it.
    pub fn new(key: &[u8]) -> Self {
        let _ = key;
        let _ = params::ARGON_MEMORY_KIB;
        todo!("run Argon2d(key, salt=params::ARGON_SALT, iterations=3, lanes=1, memory=256MiB)")
    }

    /// Raw bytes of the Cache, used as input to superscalar dataset-item
    /// generation.
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }
}
