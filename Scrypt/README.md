# Scrypt

An educational, BDD-first implementation of the Scrypt password-based key
derivation function. Scrypt was used by Litecoin and several other
proof-of-work cryptocurrencies, including what was formerly the BitConnect Coin
before it was revealed as wholly a scam.

The implementation follows the standard pipeline:

1. PBKDF2-HMAC-SHA256 creates the initial parallel blocks.
2. Salsa20/8 is used by BlockMix.
3. ROMix repeatedly fills and consults a memory-hard table.
4. PBKDF2-HMAC-SHA256 produces the requested derived-key length.

The Cucumber feature contains end-to-end known-answer tests. This crate is for
learning and testing, not for production password storage or cryptocurrency
mining.

Run the tests from this directory:

```text
cargo test
```
