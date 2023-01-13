Feature: FS-2 - solution must contain a design specification
    The software's design must be specified. The design specification must enumerate
    how the software is designed, in particular:
    - What is its architecture
    - What are its main components
    - How its main components interact with each other
    - How its main components address the functional specification

    The design specification is documented in a single markdown file named `design_specification.md`
    starting with `# Design specification` and where each design item is itemized by headings (h2).
    Each item may contain a subsection `### Trace` with a single list containing
    identifiers of existing requirements, see examples below.

    The complete design specification of the software must be specified as above, and
    the design specification must only specify the software.

    Example: a valid design specification
        Given the following content in `design_specification.md`
            """
            # Design specification
            ## DS-1 - Architecture
            Something
            """
        When we check its documentation
        Then we get no error

    Example: no design specification
        Given software without any documentation
        When we check its documentation
        Then we get an error of a missing design specification

    Example: incorrect title of design specification
        Given the following content in `design_specification.md`
            """
            # Design statement
            ## DS-1 - Architecture
            Something
            """
        When we check its documentation
        Then we get an error of an incorrect header in design specification

    Example: sections of the design specification must be of the form `# DS-X - title`
        Given the following content in `design_specification.md`
            """
            # Design specification
            ## Somethinf 1 - this
            """
        When we check its documentation
        Then we get an error of an incorrect design specification

    Example: design specification with a trace to existing features
        Given the following content in `design_specification.md`
            """
        # Design specification
        ## DS-1 - Example
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
        ## TEST-1 - Example
        ### Trace
        * FS-1
            """
        When we check its documentation
        Then we get no error

    Example: design with a trace to a non-existing feature
        Given the following content in `design_specification.md`
            """
        # Design specification
        ## DS-1 - Example
        ### Trace
        * FS-1
            """
        When we check its documentation
        Then we get an error regarding a wrong trace in design
