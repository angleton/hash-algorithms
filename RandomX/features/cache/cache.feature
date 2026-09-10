Feature: RandomX Cache (Argon2d)
  The Cache is 256 MiB derived once per key via Argon2d. This spot-checks
  three specific 8-byte little-endian words read directly out of the raw
  Cache memory after initialization — the official reference values from
  src/tests/tests.cpp ("Cache initialization" test).

  Scenario Outline: Spot-checking cache memory after Argon2d initialization
    Given the key "test key 000"
    When I build the RandomX cache
    Then 8-byte word <index> of the cache should equal "<value>"

    Examples:
      | index    | value              |
      | 0        | 0x191e0e1d23c02186 |
      | 1568413  | 0xf1b62fe6210bf8b1 |
      | 33554431 | 0x1f47f056d05cd99b |
