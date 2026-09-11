//! A small, educational implementation of the Scrypt key derivation function.
//!
//! Scrypt combines PBKDF2-HMAC-SHA256 with Salsa20/8 and a memory-hard ROMix.
//! The implementation favors readable stages over production optimizations.

use sha2::{Digest, Sha256};

const BLOCK_SIZE: usize = 64;

/// Derive `output_len` bytes using Scrypt's standard parameters.
///
/// `n` must be a power of two greater than one. The function panics for
/// invalid parameters or sizes that cannot be represented in memory.
pub fn derive_key(
    password: &[u8],
    salt: &[u8],
    n: u32,
    r: u32,
    p: u32,
    output_len: usize,
) -> Vec<u8> {
    assert!(
        n > 1 && n.is_power_of_two(),
        "n must be a power of two greater than one"
    );
    assert!(r > 0 && p > 0, "r and p must be positive");

    let block_len = 128usize.checked_mul(r as usize).expect("r is too large");
    let parallel_len = block_len
        .checked_mul(p as usize)
        .expect("p and r are too large");

    // The first PBKDF2 pass supplies one independent ROMix input per lane.
    let mut b = pbkdf2_hmac_sha256(password, salt, parallel_len);

    for block in b.chunks_exact_mut(block_len) {
        romix(block, n as usize, r as usize);
    }

    // The second PBKDF2 pass turns the memory-hard intermediate into the key.
    pbkdf2_hmac_sha256(password, &b, output_len)
}

fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut key_block = [0u8; 64];
    if key.len() > key_block.len() {
        key_block[..32].copy_from_slice(&Sha256::digest(key));
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }

    let mut inner = Vec::with_capacity(64 + message.len());
    inner.extend(key_block.iter().map(|byte| byte ^ 0x36));
    inner.extend_from_slice(message);
    let inner_hash = Sha256::digest(&inner);

    let mut outer = Vec::with_capacity(64 + inner_hash.len());
    outer.extend(key_block.iter().map(|byte| byte ^ 0x5c));
    outer.extend_from_slice(&inner_hash);
    Sha256::digest(&outer).into()
}

fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], output_len: usize) -> Vec<u8> {
    let block_count = output_len.div_ceil(32);
    let mut output = Vec::with_capacity(block_count * 32);

    for block_index in 1..=block_count {
        let index = u32::try_from(block_index).expect("derived key is too large");
        let mut input = Vec::with_capacity(salt.len() + 4);
        input.extend_from_slice(salt);
        input.extend_from_slice(&index.to_be_bytes());
        output.extend_from_slice(&hmac_sha256(password, &input));
    }

    output.truncate(output_len);
    output
}

fn romix(block: &mut [u8], n: usize, r: usize) {
    let block_len = 128 * r;
    assert_eq!(block.len(), block_len);

    let mut x = block.to_vec();
    let mut v = Vec::with_capacity(n * block_len);

    // Fill V with the sequential states; this is the memory-hard portion.
    for _ in 0..n {
        v.extend_from_slice(&x);
        x = block_mix(&x, r);
    }

    // Revisit pseudo-random prior states before the final BlockMix step.
    for _ in 0..n {
        let j = integerify(&x, r) & (n - 1);
        for (left, right) in x.iter_mut().zip(&v[j * block_len..(j + 1) * block_len]) {
            *left ^= right;
        }
        x = block_mix(&x, r);
    }

    block.copy_from_slice(&x);
}

fn integerify(block: &[u8], r: usize) -> usize {
    let offset = (2 * r - 1) * BLOCK_SIZE;
    u64::from_le_bytes(block[offset..offset + 8].try_into().unwrap()) as usize
}

fn block_mix(block: &[u8], r: usize) -> Vec<u8> {
    let mut x = [0u8; BLOCK_SIZE];
    x.copy_from_slice(&block[(2 * r - 1) * BLOCK_SIZE..2 * r * BLOCK_SIZE]);
    let mut y = vec![0u8; block.len()];

    // Apply Salsa20/8 to each 64-byte chunk, then separate even and odd chunks.
    for i in 0..2 * r {
        for byte in 0..BLOCK_SIZE {
            x[byte] ^= block[i * BLOCK_SIZE + byte];
        }
        salsa20_8(&mut x);
        y[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&x);
    }

    let mixed = y.clone();
    for i in 0..r {
        y[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE]
            .copy_from_slice(&mixed[(2 * i) * BLOCK_SIZE..(2 * i + 1) * BLOCK_SIZE]);
        y[(r + i) * BLOCK_SIZE..(r + i + 1) * BLOCK_SIZE]
            .copy_from_slice(&mixed[(2 * i + 1) * BLOCK_SIZE..(2 * i + 2) * BLOCK_SIZE]);
    }
    y
}

fn salsa20_8(block: &mut [u8; BLOCK_SIZE]) {
    let input: [u32; 16] = std::array::from_fn(|index| {
        u32::from_le_bytes(block[index * 4..index * 4 + 4].try_into().unwrap())
    });
    let mut x = input;

    // Four column/row double-rounds provide Salsa20/8's eight rounds.
    for _ in 0..4 {
        x[4] ^= (x[0].wrapping_add(x[12])).rotate_left(7);
        x[8] ^= (x[4].wrapping_add(x[0])).rotate_left(9);
        x[12] ^= (x[8].wrapping_add(x[4])).rotate_left(13);
        x[0] ^= (x[12].wrapping_add(x[8])).rotate_left(18);
        x[9] ^= (x[5].wrapping_add(x[1])).rotate_left(7);
        x[13] ^= (x[9].wrapping_add(x[5])).rotate_left(9);
        x[1] ^= (x[13].wrapping_add(x[9])).rotate_left(13);
        x[5] ^= (x[1].wrapping_add(x[13])).rotate_left(18);
        x[14] ^= (x[10].wrapping_add(x[6])).rotate_left(7);
        x[2] ^= (x[14].wrapping_add(x[10])).rotate_left(9);
        x[6] ^= (x[2].wrapping_add(x[14])).rotate_left(13);
        x[10] ^= (x[6].wrapping_add(x[2])).rotate_left(18);
        x[3] ^= (x[15].wrapping_add(x[11])).rotate_left(7);
        x[7] ^= (x[3].wrapping_add(x[15])).rotate_left(9);
        x[11] ^= (x[7].wrapping_add(x[3])).rotate_left(13);
        x[15] ^= (x[11].wrapping_add(x[7])).rotate_left(18);

        x[1] ^= (x[0].wrapping_add(x[3])).rotate_left(7);
        x[2] ^= (x[1].wrapping_add(x[0])).rotate_left(9);
        x[3] ^= (x[2].wrapping_add(x[1])).rotate_left(13);
        x[0] ^= (x[3].wrapping_add(x[2])).rotate_left(18);
        x[6] ^= (x[5].wrapping_add(x[4])).rotate_left(7);
        x[7] ^= (x[6].wrapping_add(x[5])).rotate_left(9);
        x[4] ^= (x[7].wrapping_add(x[6])).rotate_left(13);
        x[5] ^= (x[4].wrapping_add(x[7])).rotate_left(18);
        x[11] ^= (x[10].wrapping_add(x[9])).rotate_left(7);
        x[8] ^= (x[11].wrapping_add(x[10])).rotate_left(9);
        x[9] ^= (x[8].wrapping_add(x[11])).rotate_left(13);
        x[10] ^= (x[9].wrapping_add(x[8])).rotate_left(18);
        x[12] ^= (x[15].wrapping_add(x[14])).rotate_left(7);
        x[13] ^= (x[12].wrapping_add(x[15])).rotate_left(9);
        x[14] ^= (x[13].wrapping_add(x[12])).rotate_left(13);
        x[15] ^= (x[14].wrapping_add(x[13])).rotate_left(18);
    }

    for (word, original) in x.iter_mut().zip(input) {
        *word = word.wrapping_add(original);
    }
    for (index, word) in x.into_iter().enumerate() {
        block[index * 4..index * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
}
