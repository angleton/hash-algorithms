Feature: SHA-256 block word parsing
  Each 512-bit message block is read as sixteen big-endian 32-bit words.

  Scenario Outline: Parsing selected words from a padded block
    Given the 512-bit block "61626380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018"
    When I parse the SHA-256 block words
    Then word <index> should equal "<value>"

    Examples:
      | index | value      |
      | 0     | 0x61626380 |
      | 1     | 0x00000000 |
      | 15    | 0x00000018 |