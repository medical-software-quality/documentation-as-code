This repository contains a specification to document medical software
compliant with EMA[^1][^2] and FDA[^3][^4] requirements in a modern DevOps context
with git, continuous integration and continuous deployment (CI/CD).

It also contains a CLI application and docker image to check that a documentation fulfills
this specification.

[^1]: [Guidance on Qualification and Classification of Software in Regulation (EU) 2017/745 – MDR and Regulation (EU) 2017/746 – IVDR](https://health.ec.europa.eu/system/files/2020-09/md_mdcg_2019_11_guidance_qualification_classification_software_en_0.pdf)

[^2]: [Draft Guideline on computerised systems and electronic data in clinical trials](https://www.ema.europa.eu/en/documents/regulatory-procedural-guideline/draft-guideline-computerised-systems-electronic-data-clinical-trials_en.pdf)

[^3]: [General Principles of Software Validation](https://www.fda.gov/regulatory-information/search-fda-guidance-documents/general-principles-software-validation)

[^4]: [Draft Computer Software Assurance for Production and Quality System Software](https://www.fda.gov/regulatory-information/search-fda-guidance-documents/computer-software-assurance-production-and-quality-system-software)

# Background

Quality assurance (QA) in medical software is a meticulous process comprising
a significant number of hand-held activities
that produce documentation with the aim at establishing intended use, risk mitigation,
and fitness for intended use.

Modern software is developed and released under CI/CD, which is a largely
automated process of installing, verifying and deploying software.
Furthermore, modern software development puts a strong emphasis on risks derived from
project planning, management and mis-alignment between
user requirements and functional specification with methodologies such as Agile and behavior
driven development (BDD).

Almost paradoxically, although these processes seem incompatible in form, their objectives are
the same: improve quality and reduce risk using a risk-based approach.

## One organization with two practices

The primary consequence of these two cultures is that organizations
usually have 2 teams working in QA - "software QA", comprising
of people with expertise in software testing and DevOps, and "process QA", comprising
of validation leads and QA leads with expertise in the requirements set out by
governamental agencies. These teams usually produce two parallel sets of deliverables:

* The process description executed by "software QA" (hereby named "process")
* The interpreted process description by "process QA" (hereby named "shadow process")

The former comprises the practices used by the team developing and operating software solutions.
These are for example pull request markdown templates, yaml files describing the CI/CD, etc.
The latter comprises a set of documents, SOPs, and risk assessments that the "QA organization"
deemed necessary based on its interpretation of the requirements set out by governamental agencies.

Very often, the shadow process drifts from the process because developers do
not _use_ the documentation of the shadow process. Consequently,
when presenting the process in an inspection, the "process QA" is not presenting the process,
but rather a shadow of the actual process.

Overall, this makes audits and inspections assess a shadow process, resulting in
a missed opportunity by all sides (development team, QA team and auditor) to improve
overall patient safety, product quality and data integrity.

## This repository - documentation as code

This repository contains [a specification](./documentation/features/README.md) to
document software aimed at bridging the gap between the QA practiced by
the pharmaceutical industry
(governed by the corresponding governmantal bodies such as FDA and EMA)
and the state of the art practices used in the software industry.

By adereing to this specification, documentation management becomes part
of the development process, enabling the process of producing, versioning,
and publishing documentation to be part of the software development process.

Bridging these, this repository contains a command line interface (CLI) that
can be used in CI/CD to incorporate these practices in the development process.
This CLI is documented according to the specification outlined in this repository,
therefore also serving as a example to how to use the specification.

Note: technologically, documenting software to follow the specification
in this repository is "trivial". Often the barrier is found the teams'
organization and knowledge and this repository is therefore aimed at reducing
this barrier, not solving a difficult technical problem.

## For software developers

If you are a software developer, DevOps engineer, or tech lead concerned that your QA is requesting too many word documents, excel sheets, wiki pages or PDFs
that you know will either:
* drift from what the software is or
* have a significant cost and/or quality of life impact to your team.

This repository helps at formulating a proposal to your QA with a mechanism to document
the software that is:
* developer friendly (git, markdown, Gherkin)
* aligned with current DevOps practices (documentation lives next to code and is checked in CI/CD)
* compatible with QA's requirements:
    * good documentation practices (who, when, what, identifiers and document versioning)
    * software documentation required for "custom software"
* extensible to your organization's specific requirements

By adopting this specification, you are inviting your QA or validation lead to become
part of the DevOps process whereby documentation, just like working tests, is a 
requirement for releasable software.

## For validation leads

If you are a QA or validation lead concerned that developers do not engage in producing
the necessary documentation of the software they develop, resulting in the "shadow process"
described above, this repository helps you at formulating a proposal to your team with a mechanism
to document the solution they will significantly improve the collaboration and engagement
of your team in the process, resulting in a overall higher quality of the software and its
associated documentation.

# How to use

See [user manual](./documentation/user_manual.md).

# Contribute and development notes

This is a standard Rust project.

To run tests:

```bash
cargo test
```

To run it against a directory

```bash
cargo run --path /documentation
```

To build and run image against this repository

```bash
docker build . -t quality
docker run -v `pwd`/documentation:/documentation --rm quality --path /documentation
```

# Versioning

This repository uses semantic versioning.

# License

This repository and all files on it are licensed according to the [LICENSE](LICENSE.md).

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the license, shall be licensed as above, without any additional terms or conditions.
