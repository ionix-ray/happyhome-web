Feature: Brand Mode Toggle
  Scenario: Default mode is Hospital
    Given the site loads
    Then the brand mode is "Hospital"
    And the page shows hospital tagline
    And the accent color is Medical Blue

  Scenario: Toggle to Rehab mode
    Given the site is in Hospital mode
    When I click the brand toggle to "Rehab"
    Then the brand mode is "Rehab"
    And the page shows rehab tagline "Helping You To Bring Back Your Smile"
    And the rehab services are displayed
    And the team section shows rehab team

  Scenario: Toggle back to Hospital mode
    Given the site is in Rehab mode
    When I click the brand toggle to "Hospital"
    Then the brand mode is "Hospital"
    And hospital content is displayed
