Feature: Contact Form Validation
  Background:
    Given the appointment form is loaded

  Scenario: Valid form submission succeeds
    Given I fill in "Full Name" with "Rahul Mohanty"
    And I fill in "Phone" with "+91 8249914673"
    And I fill in "Email" with "rahul@example.com"
    And I fill in "Subject" with "Post Surgery Rehab"
    When I click "Submit"
    Then I see a success toast message

  Scenario: Empty name shows error
    Given I leave "Full Name" empty
    And I fill in "Phone" with "+91 9876543210"
    And I fill in "Email" with "test@example.com"
    When I click "Submit"
    Then I see validation error "Name is required"

  Scenario: Invalid email shows error
    Given I fill in "Full Name" with "Priya Patra"
    And I fill in "Phone" with "+91 9876543210"
    And I fill in "Email" with "not-an-email"
    When I click "Submit"
    Then I see validation error "Please enter a valid email address"

  Scenario: Invalid phone shows error
    Given I fill in "Full Name" with "Priya Patra"
    And I fill in "Phone" with "123"
    And I fill in "Email" with "priya@example.com"
    When I click "Submit"
    Then I see validation error "Please enter a valid phone number"
