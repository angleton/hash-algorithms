# Hash Algorithms

Educational, from-scratch implementations of hashing algorithms, built
incrementally using BDD (Behavior-Driven Development) — each algorithm's
individual steps are described as Gherkin scenarios with known correct
inputs/outputs before being implemented, in addition to a full end-to-end
test of the whole algorithm.

## Algorithms

- [**RandomX**](RandomX/) — the memory-hard, CPU-friendly proof-of-work
  hash algorithm used by Monero, with a working light-mode implementation
  and CPU/memory telemetry. See [RandomX/README.md](RandomX/README.md).
- [**SHA-256**](SHA256/) — a BDD-first scaffold for the standard SHA-256
  pipeline. The feature files cover each step, while the implementation is
  intentionally left pending for comparison against older code.
- [**Scrypt**](Scrypt/) — an educational implementation of the memory-hard
  password-based key derivation function used by Litecoin and other coins.

More algorithms will be added here over time, each in their own top-level
directory.
