Feature: FS-1 - solution must contain a risk assessment
  Scenario: solution does not contain documentation/risk_assessment.md
    Given a solution without any documentation
    When we check its documentation
    Then we get an error of a missing risk assessemnt file

  Scenario: solution contains documentation/risk_assessment.md
    Given a solution with a valid risk assessment
    When we check its documentation
    Then we get no error
