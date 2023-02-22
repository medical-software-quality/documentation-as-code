Feature: FS-6 - solution must contain a user manual
    The software's user manual must exist.

    The user manual is documented in a single markdown file named `user_manual.md`
    starting with `# User manual` and where each persona is itemized by headings (h2).

    Besides the software itself, this manual must be the only place where usage is described, and
    the user manual must only describe how to use this software.

    Example: a valid user manual
        Given the following content in `user_manual.md`
            """
            # User manual
            ## USER-1 - Principal Investigator

            As a principal investigator, you access the overview of your sites via "site overview" on the
            header of the initial page.

            If pressed, you can search, filter and sort present and past study deviations. Select individual
            deviations to open their audit trail.
            """
        When we check its documentation
        Then we get no error

    Example: no user manual
        Given software without any documentation
        When we check its documentation
        Then we get an error of a missing user manual file

    Example: headings of the user manual must be of the form `# USER-X - title`
        Given the following content in `user_manual.md`
            """
            # User manual
            ## STEP-1 - this
            """
        When we check its documentation
        Then we get an error of an incorrect user manual

    Example: user manual with a trace to existing feature
        Given the following content in `user_manual.md`
            """
        # User manual
        ## USER-1 - Example
        ### Trace
        * FS-1
            """
        And the following content in `risk_assessment.md`
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
        And the following content in `verification_plan.md`
            """
        # Verification plan
        ## TEST-1 - Placeholder
        ### Trace
        * FS-1
            """
        When we check its documentation
        Then we get no error

    Example: user manual with a trace to a non-existing feature
        Given the following content in `user_manual.md`
            """
        # User manual
        ## USER-1 - Example
        ### Trace
        * FS-1
            """
        When we check its documentation
        Then we get an error regarding a wrong trace in user manual
