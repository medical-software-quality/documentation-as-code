Feature: FS-2 - solution must contain a risk assessment
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

  The risk assessment is documented in a single markdown file, itimized by headings (h1).
  See examples below.

  Example: solution with a valid documentation/risk_assessment.md
    Given the following valid risk assessment
      """
      # RISK-1 - Risk of outdated records prior to submission

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

  Scenario: solution without documentation/risk_assessment.md
    Given a solution without any documentation
    When we check its documentation
    Then we get an error of a missing risk assessemnt file
