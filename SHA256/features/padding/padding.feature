Feature: SHA-256 message padding
  SHA-256 appends one set bit, enough zero bits to leave 64 bits at the end
  of the final block, then the original message length as a big-endian u64.

  Scenario: Padding the empty message
    Given the message ""
    When I pad the SHA-256 message
    Then the padded message should equal "80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"

  Scenario: Padding abc into one block
    Given the message "abc"
    When I pad the SHA-256 message
    Then the padded message should equal "61626380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018"