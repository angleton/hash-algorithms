Feature: Derive a Scrypt key
  Scrypt combines PBKDF2, Salsa20/8, and a memory-hard ROMix to derive a key.

  Scenario Outline: Deriving keys from known vectors
    Given the password "<password>" and salt "<salt>"
    And the Scrypt parameters N <n>, r <r>, p <p>, and output length <length>
    When I derive the Scrypt key
    Then the resulting key should equal "<key>"

    Examples:
      | password | salt | n  | r | p | length | key |
      |          |      | 16 | 1 | 1 | 64     | 77d6576238657b203b19ca42c18a0497f16b4844e3074ae8dfdffa3fede21442fcd0069ded0948f8326a753a0fc81f17e8d3e0fb2e0d3628cf35e20c38d18906 |
      | pleaseletmein | SodiumChloride | 16 | 1 | 2 | 64 | 4906a835dadabea036bdb0bddbc7687ff7674613ab7a858669323554d1986b5b39ff8ca4806612db046122e0cacf03067a73831a3e5c6dbba950fdef5a35c917 |