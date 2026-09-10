Feature: SHA-256 message schedule expansion
  The first sixteen words from the block are expanded into sixty-four words
  using the small sigma functions and modular addition.

  Scenario Outline: Expanding selected schedule words for abc
    Given the first 16 schedule words "61626380,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000000,00000018"
    When I expand the SHA-256 message schedule
    Then schedule word <index> should equal "<value>"

    Examples:
      | index | value      |
      | 16    | 0x61626380 |
      | 17    | 0x000f0000 |