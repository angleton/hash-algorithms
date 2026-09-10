Feature: SHA-256 compression round
  One round updates the eight working variables from one schedule word and
  one round constant.

  Scenario: Running the first round for the abc block
    Given the working state "6a09e667,bb67ae85,3c6ef372,a54ff53a,510e527f,9b05688c,1f83d9ab,5be0cd19"
    And the schedule word "0x61626380"
    And the round constant "0x428a2f98"
    When I run one SHA-256 compression round
    Then the working state should equal "5d6aebcd,6a09e667,bb67ae85,3c6ef372,fa2a4622,510e527f,9b05688c,1f83d9ab"