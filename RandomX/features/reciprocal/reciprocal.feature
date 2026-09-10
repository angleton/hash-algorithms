Feature: RandomX reciprocal (the IMUL_RCP helper)
  IMUL_RCP multiplies a register by the modular reciprocal of a fixed
  32-bit divisor instead of dividing, so RandomX programs never execute a
  division instruction. `reciprocal(divisor)` computes that 64-bit
  constant. These are the official reference values from
  src/tests/tests.cpp ("randomx_reciprocal" test).

  Scenario Outline: Computing the reciprocal of a divisor
    Given the divisor <divisor>
    When I compute its RandomX reciprocal
    Then the reciprocal should equal <reciprocal>

    Examples:
      | divisor    | reciprocal           |
      | 3          | 12297829382473034410 |
      | 13         | 11351842506898185609 |
      | 33         | 17887751829051686415 |
      | 65537      | 18446462603027742720 |
      | 15000001   | 10316166306300415204 |
      | 3845182035 | 10302264209224146340 |
      | 4294967295 | 9223372039002259456  |
