Feature: Compute a SHA-256 hash
  SHA-256 turns a byte message into a 256-bit digest through preprocessing,
  message schedule expansion, compression, and final state serialization.

  Scenario Outline: Hashing known messages
    Given the message "<message>"
    When I calculate the SHA-256 hash
    Then the resulting hash should equal "<digest>"

    Examples:
      | message | digest                                                           |
      |         | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |
      | abc     | ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad |