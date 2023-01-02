Feature: FS-1 - solution specification are Gherkin feature files
  Scenario: solution's specification is in documentation/features
    Given a solution
    When it contains a directory `documentation/features`
    Then specification is presented
