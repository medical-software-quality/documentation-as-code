Feature: FS-5 - output to JSON
    If no error is found, this solution must output the verified
    software's documentation as a valid JSON.

    Example: valid documentation
        Given the following content in `design_specification.md`
            """# Design specification
## DS-1 - Architecture
Something
            """
        And the following content in `user_manual.md`
            """# User manual
## USER-1 - developer
            """
        And the following content in `risk_assessment.md`
            """# Risk assessment
## RISK-1 - example
            """
        And the following verification plan
            """# Verification plan
## TEST-1 - example
### Trace
* FS-1
            """
        And the following feature
            """Feature: FS-1 - All user actions that add or modify data must be logged to form an audit trail
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
        When we check its documentation
        Then we get the following JSON
            """
            {
                "requirements": {
                    "FS-1": "Feature: FS-1 - All user actions that add or modify data must be logged to form an audit trail\n    The log entry must include:\n    - who (against company X's AD)\n    - when (timestamp in UTC)\n    - what (what is the data after the action)\n    See FS-2 regarding hard and soft deletes.\n\n    Scenario: Log action that modifies data\n        Given a user\n        And the user is not authenticated\n        When the user tries to access the system\n        Then it must not be allowed to\n"
                },
                "design_specification": {
                    "text": "# Design specification\n## DS-1 - Architecture\nSomething\n",
                    "trace": {
                        "DS-1": []
                    }
                },
                "risk_assessment": {
                    "text": "# Risk assessment\n## RISK-1 - example\n",
                    "trace": {
                        "RISK-1": []
                    }
                },
                "verification_plan": {
                    "text": "# Verification plan\n## TEST-1 - example\n### Trace\n* FS-1\n",
                    "trace": {
                        "TEST-1": [
                            "FS-1"
                        ]
                    }
                },
                "user_manual": {
                    "text": "# User manual\n## USER-1 - developer\n",
                    "trace": {
                        "USER-1": []
                    }
                }
            }
            """
