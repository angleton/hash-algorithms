# RandomX Miner (from scratch, in Rust)

An educational, incremental Rust implementation of **RandomX**, the
proof-of-work hash algorithm Monero has used since November 2019. The goal
of this repo is to understand RandomX deeply by building it up one concept
at a time — starting with computing a single hash correctly, and only
later worrying about making it fast enough to mine with.

Development follows **BDD** (Behavior-Driven Development): each stage of
the algorithm is first described as a Gherkin scenario with a known,
official expected result, and only then implemented until that scenario
goes green.

## Why RandomX?

Most proof-of-work algorithms (like Bitcoin's SHA-256) are trivial to
implement efficiently in dedicated hardware (ASICs), which centralizes
mining around whoever can afford that hardware. RandomX is designed
specifically to resist this: it dynamically generates and executes
semi-random *programs* using general-purpose CPU instructions (integer
math, floating point math, branches) and leans on large, slow memory
("memory-hard") so that a regular CPU is close to the most efficient way
to compute it. There is no fixed circuit to bake into an ASIC — every hash
runs different code.

## Project status

The whole single-hash pipeline is scaffolded and documented (see
[`src/lib.rs`](src/lib.rs) and the module list below), but only the
*shape* of the algorithm exists so far — each stage is a `todo!()` stub.
The BDD suite in [`features/randomx_hash.feature`](features/randomx_hash.feature)
currently fails ("red") at the first unimplemented stage (Argon2d cache
generation). See the todo list in this repo's tracked issues/PRs for
current progress; the short version is:

- [x] Project + BDD scaffold
- [ ] `Cache` (Argon2d)
- [ ] `SuperscalarHash` program generation
- [ ] Dataset item derivation (light mode)
- [ ] AES-based scratchpad/entropy generation
- [ ] Program generation/decoding (the random instruction stream)
- [ ] VM instruction execution
- [ ] Final Blake2b + wiring it all together in `calculate_hash`

## How a single RandomX hash is computed

`calculate_hash(key, input) -> [u8; 32]` is the entire algorithm. `key`
changes rarely (in Monero, roughly every 2048 blocks — about every 2.8
days); `input` is the per-attempt block header + nonce being hashed. A real
miner reuses the expensive `key`-derived state (the Cache/Dataset) across
millions of `input` values; this project starts by treating the whole
thing as one call so the mechanics are clear, before optimizing reuse.

### 1. Cache — turning the key into 256 MiB of noise

*Module: [`src/cache.rs`](src/cache.rs)*

RandomX runs [Argon2d](https://en.wikipedia.org/wiki/Argon2) (the
data-dependent variant of the password-hashing function Argon2) over the
key to produce a 256 MiB block of pseudo-random bytes called the **Cache**.
Argon2d itself is memory-hard and slow by design, which is exactly why
RandomX reuses this result across many hashes rather than recomputing it
every time.

Fixed parameters (see [`src/params.rs`](src/params.rs)):
- password = the key `K`
- salt = the literal bytes `"RandomX\x03"`
- 3 iterations, 1 lane (no parallelism), 262144 KiB (256 MiB) of memory

### 2. SuperscalarHash — expanding the Cache into a Dataset on demand

*Module: [`src/superscalar.rs`](src/superscalar.rs)*

The *real* memory-hard structure RandomX wants you to have is the
**Dataset**: over 2 GiB of pseudo-random data. Keeping all 2 GiB resident
is called **fast mode** — it's what a real miner uses because it's fast.
But requiring 2 GiB just to *verify* one hash would be unfriendly to
lightweight nodes, so RandomX also defines **light mode**: instead of
storing the Dataset, each 64-byte Dataset item is derived on the fly from
the 256 MiB Cache. Both modes are required to produce identical hashes.

The derivation isn't just "hash a chunk of the Cache" (too cheap to be
memory-hard on its own) — it mixes 8 pseudo-random reads from the Cache
through one of 8 generated **superscalar programs**: short, fixed
sequences of integer instructions (multiply, add, xor, rotate...) designed
by RandomX's authors to require significant *latency* (dependent
instruction chains) to compute, not just memory bandwidth. This is what
makes light mode slow but low-memory, and fast mode fast but memory-hungry
— both arrive at the same answer.

This project implements **light mode first**, since it's simpler to get
correct (only needs the 256 MiB Cache), even though it's much slower.

### 3. Seeding the first program

`input` is hashed (Blake2b) together with a small amount of fixed context
to produce the first 128-byte "entropy" buffer. Every subsequent program's
entropy buffer instead comes from the *previous* program's execution
(step 4d below) — this is what chains the 8 programs together into one
hash, and why RandomX can't be parallelized across programs within a
single hash.

### 4. Running 8 chained programs

*Modules: [`src/program.rs`](src/program.rs), [`src/vm.rs`](src/vm.rs),
[`src/aes_generator.rs`](src/aes_generator.rs)*

RandomX executes `RANDOMX_PROGRAM_COUNT` = **8** programs back-to-back,
each built and torn down using the previous program's leftover state.
For each of the 8 rounds:

**a. Generate the program.** The current 128-byte entropy buffer is
expanded (via `AesGenerator1R`, a fixed-key AES round function used purely
as a fast bit mixer, not for secrecy) into 256 instructions of 8 bytes
each — an opcode byte, two operand/register bytes, and a 4-byte immediate.
Each instruction decodes into one of roughly 30 operation types: integer
add/sub/mul/xor/rotate, floating point add/sub/mul/div/sqrt, conditional
branches, and scratchpad loads/stores. Because the entropy buffer differs
every round, the *program itself* — not just the data — is different every
time. That's the "random" in RandomX.

**b. Load the register file.** 8 signed 64-bit integer registers
(`r0..r7`) and 12 floating point registers (grouped `f0..f3`, `e0..e3`,
`a0..a3`) are initialized from the same entropy buffer. Two of the integer
registers (`ma`/`mx`) hold scratchpad addresses used for reads/writes.

**c. Fill/mix the scratchpad and execute.** The 2 MiB **scratchpad** (the
VM's private, per-hash working memory — much smaller than the Cache/
Dataset, and addressed at three granularities: 16 KiB / 256 KiB / 2 MiB
windows, `L1`/`L2`/`L3` in [`src/params.rs`](src/params.rs)) is
AES-filled at the start of a hash. The program's 256 instructions then run
in a loop for `RANDOMX_PROGRAM_ITERATIONS` = **2048** iterations. Each
iteration executes all 256 instructions in sequence, reading and writing
scratchpad bytes at addresses derived from the integer registers, and
periodically pulling a 64-byte item out of the Dataset (step 2) to mix in
— this is the actual memory-hard access pattern that makes the algorithm
expensive to shortcut.

**d. "Hash and fill".** After the 2048 iterations, the register file is
mixed into the scratchpad via `AesGenerator4R` (four AES rounds per step
instead of one — slower, used here instead of at scratchpad-fill time
because this step also has to *consume* the existing scratchpad content,
not just produce new bytes). The output doubles as the entropy buffer for
the *next* program, chaining round `i` into round `i+1`.

### 5. Finalizing the hash

After the 8th program finishes, the final register file (all int + float
registers) is hashed with **Blake2b** to produce the 32-byte digest. That
digest is the RandomX hash — what Monero compares against the network
difficulty target.

## Repository layout

```
src/
  lib.rs             Top-level calculate_hash() and the full pipeline walkthrough
  params.rs          Fixed RandomX constants (from the official spec)
  cache.rs           Stage 1: Argon2d-derived Cache
  superscalar.rs      Stage 2: SuperscalarHash program generation
  dataset.rs          Stage 2b: on-demand Dataset item derivation (light mode)
  aes_generator.rs    Stage 4: AesGenerator1R / AesGenerator4R
  program.rs          Stage 4a: entropy -> decoded instruction program
  vm.rs               Stage 4b-d: register file, scratchpad, execution loop
  main.rs             Thin CLI entry point (prints one hash)
features/
  randomx_hash.feature  BDD scenarios, using official RandomX test vectors
tests/
  cucumber.rs           Step definitions wiring Gherkin steps to calculate_hash()
```

## Running the tests

```powershell
cargo test --test cucumber
```

This runs the Gherkin scenarios in `features/randomx_hash.feature` against
`calculate_hash`. Scenarios use the official reference test vectors from
the upstream [RandomX repository](https://github.com/tevador/RandomX)'s
test suite (`src/tests/tests.cpp`), computed with the classic
("v1"/interpreter) variant of the algorithm, so a fully correct
implementation should make every scenario pass without any test changes.

## Running the (eventual) binary

```powershell
cargo run
```

Currently just computes one hash of a hardcoded key/input and prints it in
hex — a placeholder until the algorithm is implemented and a real CLI
(key/input/nonce-range arguments, hashrate reporting, etc.) is built on
top of it.

## References

- [RandomX repository (tevador/RandomX)](https://github.com/tevador/RandomX) — the reference C++ implementation and specification (`doc/specs.md`, `doc/design.md`)
- [Monero's RandomX proof-of-work documentation](https://www.getmonero.org/resources/moneropedia/randomx.html)
