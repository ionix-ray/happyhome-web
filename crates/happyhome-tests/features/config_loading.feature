Feature: TOML Configuration Loading
  Scenario: Site config loads successfully
    Given the site.toml file is valid
    When the config is loaded
    Then site name is "Happy Home Hospital And Rehab"
    And contact phone is "+91 8249914673"
    And contact email is "hello@happyhomehospital.com"
    And rehab has 9 services
    And rehab team has 6 members

  Scenario: Rehab services are complete
    Given the config is loaded
    When I inspect rehab services
    Then the service list includes "Stroke Recovery & Paralysis Management"
    And the service list includes "Traumatic Brain Injury (TBI) Care"
    And the service list includes "Post Neuro-Surgery Rehabilitation"

  Scenario: Footer config is complete
    Given the config is loaded
    Then footer quick links include "About Us"
    And footer legal links include "Privacy Policy"
    And footer about text is not empty
