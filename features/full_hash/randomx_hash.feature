Feature: Compute a single RandomX hash (full pipeline, all steps combined)
  RandomX (used by Monero's proof of work) turns a key K and an input
  message into a 256-bit hash. This is the end-to-end integration test:
  it exercises every stage in features/ (cache, dataset, aes_generator,
  reciprocal, ...) chained together, the same way a real hash is computed.

  The expected outputs below are the official reference test vectors from
  the RandomX repository (src/tests/tests.cpp), computed with the classic
  ("v1"/interpreter) variant of the algorithm.

  Scenario: Hashing a short test string
    Given the key "test key 000"
    And the input "This is a test"
    When I calculate the RandomX hash
    Then the resulting hash should equal "639183aae1bf4c9a35884cb46b09cad9175f04efd7684e7262a0ac1c2f0b4e3f"

  Scenario: Hashing a different input with the same key
    Given the key "test key 000"
    And the input "Lorem ipsum dolor sit amet"
    When I calculate the RandomX hash
    Then the resulting hash should equal "300a0adb47603dedb42228ccb2b211104f4da45af709cd7547cd049e9489c969"

  Scenario: Changing the key changes the hash, even for the same input
    Given the key "test key 000"
    And the input "This is a test"
    When I calculate the RandomX hash
    Then the resulting hash should not equal "300a0adb47603dedb42228ccb2b211104f4da45af709cd7547cd049e9489c969"
