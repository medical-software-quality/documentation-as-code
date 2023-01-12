Feature: FS-1 - software must be specified
  The software must be specified. In particular, every critical requirement,
  functional or not, must be enumerated and described.

  Each requirement must be written as a [Gherkin `.feature` file](https://cucumber.io/docs/gherkin/reference/)
  under a directory named `features`.

  The Gherkin feature must start with `FS-`, followed by ` - ` and a title.

  The complete specification of the software must be specified as above, and
  the specification must only specify the software.

  Example: valid feature title
    Given the following feature
      """
Feature: FS-1 - All user actions that add or modify data must be logged to form an audit trail
  The log entry must include:
  - who (against company X's AD)
  - when (timestamp in UTC)
  - what (what is the data after the action)
  See FS-2 regarding hard and soft deletes.

  Scenario: Log action that modifies data
    Given a user
    And the user is not authenticated
    When the user tries to access the system
    Then it must not be allowed to
      """
    When we check it
    Then we get no error regarding its specification

  Example: invalid Gherkin feature
    Given the following feature
      """FeatureXXX: FS-1 - some feature
  Scenario: Something
    When something
    Then something else
    When something else
      """
    When we check its documentation
    Then we get an error regarding wrong Gherkin

  Example: feature with a wrong title
    Given the following feature
      """
Feature: some feature
  Scenario: Something
    When something
    Then something else
      """
    When we check it
    Then we get an error regarding a wrong identifier

  Example: feature missing title
    Given the following feature
      """
Feature: FS-1
  Scenario: Something
    When something
    Then something else
      """
    When we check it
    Then we get an error regarding a wrong identifier

  Example: software is not specified
    Given software without a specification
    When we check its documentation
    Then we get an error of a missing specification
