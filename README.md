# GAMP-compliant custom software documentation

This repository contains a specification to document medical software
that is compliant with EMEA and FDA
requirements for medical software in a modern DevOps context
with continuous integration and continuous deployment (CI/CD).

It also contains a Rust library and CLI application that verifies a set of documents
against the specification.

## Background

Quality assurance (QA) in medical devices is a lengthly,
complex, and meticuolus process comprising a significant number of hand-held activities
that produce documentation with the aim of establishing intended use, risk mitigation,
and fit for intended use. These are for example described in the book
GAMP 5: A Risk-based Approach to Compliant GxP Computerized Systems.

Modern software is developed and released under CI/CD, which is a largely
automated process of installing, verifying and deploying software.
Furthermore, modern software development puts a strong emphasis on risks derived from
project planning, management and mis-alignment between
user requirements and functional specification with methodologies such as Agile and behavior
driven development (BDD).

Almost paradoxically, although these processes seem incompatible in form, their objectives are
the same: improve quality and reduce risk using a risk-based approach.

### Risk of one organization with two practices

The primary consequence of these two seemly incompatible cultures is that organizations
usually have 2 teams working in QA - "software QA", comprising
of people with expertise in software testing and DevOps, and "process QA", comprising
of validation leads and QA leads with expertise in the requirements set out by
governamental agencies such as FDA. These teams usually produce two
parallel sets of deliverables:

* The process executed by "software QA" (hereby named "process")
* The process interpreted by "process QA" (hereby named "shadow process")

The former comprises the practices used by the team developing and operating software solutions.
The later comprises a set of documents, SOPs, and risk assessments that the "QA organization"
deemed necessary based on its interpretation of the requirements set out by governamental agencies.

Very often, the shadow process drifts from the process because developers do
not _use_ the documentation of the shadow process. Consequently,
when presenting the process in an inspection, the "process QA" is not presenting the process,
but rather a shadow of the actual process.

Overall, this makes audits and inspections assess a shadow process, resulting in
a missed oportunity in all sides (development team, QA team and auditor) to improve
overall patient safety, product quality and data integrity.

### This repository

This repository contains a specification to document software aimed at bridging the gap
between the QA practiced by the pharmaceutical industry (governed by the corresponding governmantal bodies such as FDA and EMEA) and the development practices used in 
much of the software development industry.

By adereing to this specification, documentation management becomes part
of the development process, making the process of producing, versioning,
and publishing documentation part of the CI/CD process.

This repository also contains a command line interface (CLI) that can be used in CI/CD to
incorporate these practices in an automated process.

This CLI is documented according to the specification outlined in this repository,
therefore also serving as a example to how to adopt it.

Note that technologically, documenting software to follow the specification
in this repository is "trivial". Often the barrier is found the teams'
organization and knowledge and this repository is therefore aimed at reducing
this barrier, not solving a difficult technical problem.

### For software developers

If you are a software developer, DevOps engineer, or tech lead concerned that your QA is requesting too many word documents, excel sheets, wiki pages or PDFs
that you know will either:
* drift from what the software is or
* have a significant procedural cost and/or quality of life impact to your team.

This repository helps at formulating a proposal to your QA with a mechanism to document
the software that is:
* developer friendly
* aligned with current DevOps practices
* compatible with QA's requirements:
    * good documentation practices (who, when, what, identifiers and document versioning)
    * software documentation required for "custom software"
* extensible to your organization's specific requirements

By adopting this specification, you are inviting your QA or validation lead to become
part of the DevOps process whereby documentation, just like working tests, is a 
requirement for the software to be "releasable".

### For validation leads

If you are a QA or validation lead concerned that developers do not engage in producing
the necessary documentation of the software they develop, resulting in the "shadow process"
described above, this repository helps you at formulating a proposal to your team with a mechanism
to document the solution they will significantly improve the collaboration and engagement
of your team in the process, resulting in a overall higher quality of the software and its
associated documentation.

## How to use

You can use this repository in 2 ways:

* via the docker image published on github
* via the CLI installed via Rust's Cargo tool

The CLI is specified [here](./documentation/README.md) according
to this repositories' specification.

## Contribute and development notes

This is a standard Rust project.

To run tests:

```bash
cargo test
```

To build image:

```bash
docker build . -t quality
```

To run image against this repository

```bash
docker run -v `pwd`/documentation:/documentation --rm quality --path /documentation
```

## Versioning

This repository uses semantic versioning and has two versions. A version of the specification,
and a version of the CLI.

## License

This repository and all files on it are licensed according to the
licence in [LICENSE](LICENSE.md) (Apache 2.0).
