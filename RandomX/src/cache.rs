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
use argon2::{Algorithm, Argon2, Block, Params, Version};

/// The RandomX Cache: 256 MiB derived from the key via Argon2d.
#[derive(Debug)]
pub struct Cache {
    memory: Vec<u8>,
}

impl Cache {
    /// Build the Cache from a RandomX key by running Argon2d over it.
    pub fn new(key: &[u8]) -> Self {
        let argon_params = Params::new(
            params::ARGON_MEMORY_KIB,
            params::ARGON_ITERATIONS,
            params::ARGON_LANES,
            None,
        )
        .expect("RandomX Argon2 parameters must be valid");
        let block_count = argon_params.block_count();
        let argon2 = Argon2::new(Algorithm::Argon2d, Version::V0x13, argon_params);
        let mut blocks = vec![Block::new(); block_count];
        argon2
            .fill_memory(key, params::ARGON_SALT, &mut blocks)
            .expect("RandomX cache initialization failed");

        let mut memory = Vec::with_capacity(block_count * Block::SIZE);
        for block in &blocks {
            let words: &[u64] = block.as_ref();
            for word in words {
                memory.extend_from_slice(&word.to_le_bytes());
            }
        }
        Self { memory }
    }

    /// Raw bytes of the Cache, used as input to superscalar dataset-item
    /// generation.
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1_cache_initialization_spot_checks() {
        let cache = Cache::new(b"test key 000");
        let mem = cache.memory();
        assert_eq!(mem.len(), params::ARGON_MEMORY_KIB as usize * 1024);

        let read_word = |idx: usize| -> u64 {
            let slice: [u8; 8] = mem[idx * 8..idx * 8 + 8].try_into().unwrap();
            u64::from_le_bytes(slice)
        };

        assert_eq!(read_word(0), 0x191e0e1d23c02186);
        assert_eq!(read_word(1568413), 0xf1b62fe6210bf8b1);
        assert_eq!(read_word(33554431), 0x1f47f056d05cd99b);
    }
}
