# GAMP-compliant custom software documentation

This repository contains a specification to document medical software
that is compliant with EMEA and FDA
requirements for medical software in a modern DevOps context
with continuous integration and continuous deployment (CI/CD).

## Background

Quality assurance (QA) in medical devices is historically a lengthly,
complex, and meticuolus process comprising a significant number of hand-held activities
that produce documentation with the aim of establishing risks, intended use,
and fit for purpose.

Modern software is developed and released under CI/CD, which is a largely
automated process of installing, verifying and deploying software.
Furthermore, modern software development puts a strong emphasis on risks derived from
project planning, management and mis-alignment between
user requirements and functional specification with methodologies such as Agile and behavior
driven development (BDD).

Almost paradoxically, although these processes seem incompatible in shape, their objectives are
the same: improve quality and reduce risk using a risk-based approach.

## This repository

This repository contains a specification for software documentation aimed at bridging the gap
between the QA practiced by the pharmaceutical industry (governed by the corresponding governmantal bodies such as FDA and EMEA) and the development practices used in 
much of the software development space.

By adereing to this specification, two important processes in QA,
documentation management and release management, are blended in the development process,
therefore allowing them to be checked as part of the CI/CD pipelines.

This repository also contains a CLI that can be used in CI/CD to
incorporate these practices in their automated processes.

This CLI is documented according to the specification outlined in this repository,
therefore also serving as a example to how to use it. Note that this is not required
because this CLI's intended use does not fall in the categories from FDA.

Note that technologically, documenting software to follow the specification
in this repository is "trivial". Often the barrier stems in knowing what and how to document.
Think of this repository as addressing that need.

## Versioning

This repository uses semantic versioning and has two versions. A version of the specification,
and a version of the CLI.
