Feature: FS-1 - solution specification are Gherkin feature files
  Scenario: solution's specification is in documentation/features
    Given a solution in a directory without any documentation
    When its documentation is checked
    Then the check fails
