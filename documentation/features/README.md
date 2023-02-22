# Documentation as code specification

This is a specification to document medical software
compliant with GAMP 5[^1] in a modern DevOps context
with git, continuous integration and continuous deployment (CI/CD).

Under this specification, software documentation is written in
[Markdown](https://en.wikipedia.org/wiki/Markdown) and [Gherkin](https://cucumber.io/docs/gherkin/)
in the same git repository containing the CI/CD code used to deploy the software to a production environment.

This specification fulfills its own specification, meaning that it is itself written in Markdown and 
Gherkin. It is broadly composed by a set of requirements about:

* which documents must exist
* where they should be located
* how they should be structured
* what they should contain

## Required documents

* [functional specification](./features/1_specification.feature)
* [design specification](./features/2_design.feature)
* [risk assessment](./features/3_risk.feature)
* [verification plan](./features/4_test.feature)
* [user manual](./features/6_user_manual.feature)
* [retirement plan](./features/7_retirement.feature)

[^1]: [GAMP 5 Guide 2nd Edition](https://ispe.org/publications/guidance-documents/gamp-5-guide-2nd-edition)
