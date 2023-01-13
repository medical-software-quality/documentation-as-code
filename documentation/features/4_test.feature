Feature: FS-4 - software verification plan
  The software's verification plan must be described.

  The verification plan describes how fitness for intended use is established.

  The verification plan must describe
  - how every requirement (described in FS-1) is tested
  - how each of the risks (described in FS-3) mitigated by tests is mitigated
  - where is the evidence of the execution of the test stored

  The verification plan is documented in a single markdown file starting with `# Verification specification`,
  and where each test is itemized by headings (h2).
  Each test may contain a subsection `### Trace` with a single list containing
  identifiers of existing requirements or risks.
  See examples below.

  All tests of the software must be described in this verification plan, and
  this verification plan must only assess risks of this software.
  All requirements must be covered by tests.

  Example: a valid verification plan
    Given the following verification plan
      """
      # Verification plan
      ## TEST-1 - Unit tests

      This software's individual components are tested by unit tests.

      These tests are executed on an controlled environment reproducing the
      production environment of each of its individual components via docker (see also DS-2).

      These tests are executed and reported on the CI/CD pipeline and must pass
      prior to the deployment of the software in the production environment.

      These tests cover the following requirements
      ### Trace
      - FS-1

      ## TEST-2 - Integration tests
      This software's components are tested in integration with integration tests.

      This test is executed on an environment reproducing the production environment (UAT)
      through the installation of the software on that environment.

      These tests are executed and reported on the CI/CD pipeline and must pass
      prior to the deployment of the software in the production environment.

      These tests cover the following requirements
      ### Trace
      - FS-1

      ## TEST-3 - Acceptance tests
      This software's specification is tested by acceptance tests.
      These tests are enumerated as "scenarios" in the softwares' specification.

      These tests cover all specifications
      ### Trace
      - FS-1
      """
    And the following feature
      """
Feature: FS-1 - something
  Scenario: Something
    When something
    Then something else
      """
    When we check its documentation
    Then we get no error

  Example: no verification plan
    Given software without any documentation
    When we check its documentation
    Then we get an error of a missing verification plan

  Example: headings of the verification plan must be of the form `# TEST-X - title`
    Given the following verification plan
      """
      # Verification plan
      ## Test 1 - this
      """
    When we check its documentation
    Then we get an error of an incorrect verification plan

  Example: verification plan with a trace to existing features
    Given the following verification plan
      """
    # Verification plan
    ## TEST-1 - Example
    ### Trace
    * FS-1
      """
    And the following feature
      """
Feature: FS-1 - something
  Scenario: Something
    When something
    Then something else
      """
    When we check its documentation
    Then we get no error

  Example: verification plan with a trace to a non-existing feature
    Given the following verification plan
      """
    # Verification plan
    ## TEST-1 - Example
    ### Trace
    * FS-1
      """
    When we check its documentation
    Then we get an error regarding a wrong trace in verification plan
