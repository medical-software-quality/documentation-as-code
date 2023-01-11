# Risk assessment

## RISK-1 - False positives

There is a risk that this application incorrectly flags documentation has not having
an issue when it does have an issue (false negative). This risk has a low discoverability
as the user is trusting this application to perform as intended.

This risk has no impact to patient safety, product quality or data integrity; it impacts
the quality of the software's documentation.

This risk is mitigated by tests and in particular by placing a strong focus on covering 
every false negative possible by the specification.
