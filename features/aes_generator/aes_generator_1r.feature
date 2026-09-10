Feature: AesGenerator1R
  AesGenerator1R expands a 64-byte state into pseudo-random bytes using one
  fixed-key AES round per 16-byte lane. It's used to seed each program's
  register file. This is the official reference test vector from
  src/tests/tests.cpp ("AesGenerator1R" test): the input state's first 32
  bytes come from the given hex; the remaining 32 bytes start at zero.

  Scenario: Filling a 64-byte state for one output block
    Given the AES generator state "6c19536eb2de31b6c0065f7f116e86f960d8af0c57210a6584c3237b9d064dc7"
    When I run AesGenerator1R for one block
    Then the first 32 bytes of the resulting state should equal "fa89397dd6ca422513aeadba3f124b5540324c4ad4b6db434394307a17c833ab"
