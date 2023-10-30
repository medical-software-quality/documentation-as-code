Feature: FS-5 - solution must contain an operator manual
    The software's operator manual must exist to ensure that the software's fitness
    for use is guaranteed throughout the software's lifetime.

    The operator manual should include, when applicable,
    - how to perform pro-active or preventive monitoring activities
    - how to perform access review activites
    - how to perform security reviews
    - what are and how to perform standard changes (e.g. password rotation, configuration changes, etc.)

    The operator manual must contain roles and responsibilities of the
    personas operating the application.

    The operator manual is a single markdown file named `operator_manual.md`
    starting with `# Operator manual` and where each topic is itemized by headings (h2).

    This manual must be the only place where operations are described, and
    the operator manual must only describe how to operate this software.

    Example: a valid operator manual
        Given the following content in `operator_manual.md`
            """
            # Operator manual
            ## OPERATOR-1 - Tier 3

            ### User-reported issues
            Tier 3 is responsible for monitoring user-reported issues [here](https://github.com/.../issues),
            triage, and address them.

            ### Monitoring
            Tier 3 is responsible for monitoring application-errors [here](...),
            triage, and address them.
            """
        When we check its documentation
        Then we get no error

    Example: no operator manual
        Given software without any documentation
        When we check its documentation
        Then we get an error of a missing operator manual file

    Example: headings of the operator manual must be of the form `# OPERATOR-X - title`
        Given the following content in `operator_manual.md`
            """
            # Operator manual
            ## OP-1 - this
            """
        When we check its documentation
        Then we get an error of an incorrect operator manual
