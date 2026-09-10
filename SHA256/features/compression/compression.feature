Feature: SHA-256 block compression
  A message block is expanded into a schedule and mixed into the current
  chaining state with 64 compression rounds.

  Scenario: Compressing the padded abc block from the initial state
    Given the chaining state "6a09e667,bb67ae85,3c6ef372,a54ff53a,510e527f,9b05688c,1f83d9ab,5be0cd19"
    And the 512-bit block "61626380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018"
    When I compress the SHA-256 block
    Then the chaining state should equal "ba7816bf,8f01cfea,414140de,5dae2223,b00361a3,96177a9c,b410ff61,f20015ad"