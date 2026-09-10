# SHA-256

Educational, from-scratch SHA-256 scaffold built with BDD feature files first.

This crate intentionally does not implement the algorithm yet. Each module is
stubbed with `todo!` so the feature files can act as executable notes for the
steps to compare against older implementations:

1. Padding and length encoding.
2. 512-bit block parsing into big-endian words.
3. Message schedule expansion from 16 to 64 words.
4. One compression round over the working variables.
5. One-block compression into a chaining state.
6. End-to-end digest serialization.

Run the compile-only check with:

```powershell
cargo test --no-run
```

The full BDD suite is expected to fail until each `todo!` is implemented:

```powershell
cargo test
```