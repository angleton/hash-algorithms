# SHA-256

Educational, from-scratch SHA-256 scaffold built with BDD feature files first.

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