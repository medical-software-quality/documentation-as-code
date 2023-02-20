Feature: FS-7 - retirement plan
    The plan to retire the software must exist in the form of a retirement plan.

    The retirement plan must describe how the software is retired and in particular:
    - what needs to be removed upon complete decomission
    - which users must be informed
    - what data must be preserved for a lifetime greater than the software and how can that be achieved
    - what should be modified to read-only or otherwise kept to avoid un-intended data loss

    The retirement plan is documented in a single markdown file named `retirement_plan.md`
    starting with `# Retirement plan` and where each item is itemized by headings (h2).

    Example: a valid retirement plan
        Given the following retirement plan
            """# Retirement plan
      ## RETIRE-1 - API

      This solutions' API (DS-3) can be retired by removing
      * the Kubernetes service from the helm chart
      * the DNS record associated to the API
      * the application registration in Azure Active Directory
            """
        When we check its documentation
        Then we get no error

    Example: no retirement plan
        Given software without any documentation
        When we check its documentation
        Then we get an error of a missing retirement plan

    Example: headings of the retirement plan must be of the form `# RETIRE-X - title`
        Given the following retirement plan
            """# Retirement plan
## RET-1 - this
            """
        When we check its documentation
        Then we get an error of an incorrect retirement plan
