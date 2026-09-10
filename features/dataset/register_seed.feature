Feature: Dataset item register seeding
  Step 1 of generating a Dataset item (RandomX spec section 7.3): the 8
  integer registers are seeded purely from the item's number, before any
  Cache reads or SuperscalarHash execution happen. This is the simplest,
  fully independent piece of Dataset generation to get right first.

  These expected values were computed independently from the documented
  formula and constants (not copied from the reference test suite), as a
  cross-check that both this implementation and the official constants
  (verified against dataset.cpp, doc/specs.md, and superscalar-init.cpp)
  agree.

  Scenario Outline: Seeding registers from an item number
    Given the dataset item number <item_number>
    When I seed the dataset item registers
    Then register r0 should equal "<r0>"
    And register r1 should equal "<r1>"
    And register r2 should equal "<r2>"
    And register r3 should equal "<r3>"
    And register r4 should equal "<r4>"
    And register r5 should equal "<r5>"
    And register r6 should equal "<r6>"
    And register r7 should equal "<r7>"

    Examples:
      | item_number | r0                 | r1                 | r2                 | r3                 | r4                 | r5                 | r6                 | r7                 |
      | 0           | 0x5851f42d4c957f2d | 0xd95b63a71560ded1 | 0xff216df27457a76b | 0xd9774d31f3b73671 | 0x111cd1ba5b0af54f | 0xca661b94823f9321 | 0x777ba25920735255 | 0xdcd4cfdafab99a63 |
      | 1           | 0xb0a3e85a992afe5a | 0x31a97fd0c0df5fa6 | 0x17d37185a1e8261c | 0x318551462608b706 | 0xf9eecdcd8eb57438 | 0x229407e357801256 | 0x9f89be2ef5ccd322 | 0x3426d3ad2f061b14 |
      | 1000000     | 0xdc6a2a017061e46d | 0x5d60bd8b29944591 | 0x7b1ab3de48a33c2b | 0x5d4c931dcf43ad31 | 0x95270f9667fe6e0f | 0x4e5dc5b8becb0861 | 0xf3407c751c87c915 | 0x58ef11f6c64d0123 |
