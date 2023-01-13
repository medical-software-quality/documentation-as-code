Feature: FS-3 - solution must contain a risk assessment
  The software's risk must be assessed. The risk assessment must enumerate
  risks that the software has that may impact any of the following:
  - Patient safety
  - Product quality
  - Data integrity (usually submissions)
  Each of the risks must be described in terms of:
  - severity
  - discoverability
  - mitigating actions

  Note that only actual risks are required to be enumerated -
  risks that have been significantly mitigated may not be part of the assessment.
  Note that risks are always assessed against the software specification.

  The risk assessment is documented in a single markdown file starting with `# Risk assessment`,
  and where each risk item is itemized by headings (h2).
  See examples below.

  The complete risk assessment of the software must be specified as above, and
  the risk assessment must only assess risks of this software.

  Example: a valid risk assessment
    Given the following content in `risk_assessment.md`
      """
      # Risk assessment
      ## RISK-1 - Risk of outdated records prior to submission

      As per FS-10, the solution expects users to keep the records
      of all the participants they follow updated and ready for submission.

      However, the system has no mechanism to "force" users to fill out
      visit details in the moment the participant visited.

      This risk has a low discoverability and high impact and is mitigated
      by having a mandatory item in the visit's SOP for the user to fill out the visit details
      in the system, and for the users to have mandatory training on this SOP.
      """
    When we check its documentation
    Then we get no error

  Example: no risk assessment
    Given software without any documentation
    When we check its documentation
    Then we get an error of a missing risk assessment file

  Example: headings of the risk assessment must be of the form `# RISK-X - title`
    Given the following content in `risk_assessment.md`
      """
      # Risk assessment
      ## Risk 1 - this
      """
    When we check its documentation
    Then we get an error of an incorrect risk assessment

  Example: risk assessment with a trace to existing features
    Given the following content in `risk_assessment.md`
      """
    # Risk assessment
    ## RISK-1 - Example
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
    And the following verification plan
      """
        # Verification plan
        ## TEST-1 - Placeholder
        ### Trace
        * FS-1
      """
    When we check its documentation
    Then we get no error

  Example: risk assessment with a trace to a non-existing feature
    Given the following content in `risk_assessment.md`
      """
    # Risk assessment
    ## RISK-1 - Example
    ### Trace
    * FS-1
      """
    When we check its documentation
    Then we get an error regarding a wrong trace in risks
