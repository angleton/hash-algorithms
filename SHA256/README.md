# SHA-256 Educational Implementation

Educational, from-scratch SHA-256 implementation with BDD feature files and
numbered unit tests that follow the algorithm from input preprocessing to
digest serialization.

## Background

SHA-256 is one member of the SHA-2 family. SHA-2 was designed by the United
States National Security Agency (NSA) and standardized publicly by the National
Institute of Standards and Technology (NIST) as part of the Secure Hash Standard,
starting with FIPS 180-2 in 2001. It followed SHA-1 with larger digest sizes and
a stronger construction at a time when confidence in older hash functions was
beginning to narrow.

There was also a U.S. government patent on the underlying SHA-2-style design:
US6829355B2, "Device for and method of one-way cryptographic hashing," assigned
to the NSA. That patent is now expired, and SHA-256 has long been available as a
public standard implemented freely throughout cryptographic software.

The implementation is organized into these ordered stages:

1. `stage_01_padding`: append the marker, zero padding, and 64-bit length.
2. `stage_02_block_parsing`: parse 512-bit blocks into big-endian words.
3. `stage_03_schedule_expansion`: expand 16 words into the 64-word schedule.
4. `stage_04_schedule_ready`: record the completed schedule used by compression.
5. `stage_05_compression`: execute 64 rounds and update the chaining state.
6. `stage_06_digest_serialization`: encode the eight final words as 32 bytes.
7. `stage_07`: validate telemetry ordering, CPU time, and memory attribution.

## How SHA-256 Works

SHA-256 is a deterministic function, not a search algorithm. For one input,
there is exactly one 256-bit output. The implementation transforms the input
through a Merkle-Damgard compression construction:

1. **Padding** makes the message length a multiple of 512 bits. The original
	bit length is stored in the final 64 bits.
2. **Parsing** interprets each 512-bit block as sixteen big-endian 32-bit
	words.
3. **Schedule expansion** derives 64 words from those sixteen using
	rotate-right and shift operations, spreading input bits across rounds.
4. **Compression** starts with eight fixed 32-bit state words. Each of the 64
	rounds computes the `Ch` and `Maj` boolean functions, two capital-sigma
	functions, and modular additions. The result is added to the previous
	chaining state before the next block is processed.
5. **Serialization** writes the final eight words in big-endian order to make
	the 32-byte digest.

All additions are modulo $2^{32}$. SHA-256 is designed so that reversing
these transformations is computationally infeasible, but calculating a
digest in the forward direction is fast.

## Where Brute Force Happens

Mining is the search wrapped around the hash function, not a search performed
inside SHA-256. A proof-of-work system defines a target, usually an upper bound
on the numeric hash value. A miner repeatedly changes a nonce or other input:

```text
candidate_0 -> SHA-256 -> hash_0
candidate_1 -> SHA-256 -> hash_1
candidate_2 -> SHA-256 -> hash_2
...
```

The miner keeps trying candidates until one digest is below the target. The
hash behaves like a pseudorandom 256-bit value, so there is no useful shortcut
for predicting which nonce will succeed. If one digest has probability $p$ of
meeting the target, the expected number of trials is $1/p$.

Verification is different: a node receives one proposed block, computes its
hash once, and checks the target. Mining is brute-force search over inputs;
verification is one deterministic computation.

Run the full unit and BDD suite with:

```powershell
cargo test
```

Run the CLI, which prints the digest followed by stage telemetry:

```powershell
cargo run -- "abc"
```

Telemetry reports wall time, process CPU time, CPU share, and attributed
memory for each pipeline stage. Cucumber scenarios are acceptance tests; the
numbered `#[test]` functions in `src/` verify the intermediate implementation
steps and reference vectors.