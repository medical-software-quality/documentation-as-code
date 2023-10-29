# User manual

## USER-1 - how to use

1. Write the software's documentation using the specification described [here](./features/README.md) in your code repository
2. Add a step to your CI to verify the documentation (via the CLI in this repository)
3. Put guard rails on the development process. For example:
    * code ownership policy so that QC is required to review changes to it
    * pull request review guidelines requesting developers to ensure that documentation is updated
    * [semantic versioning](https://semver.org/) in place to control for backward incompatible
      changes to the specification

Note that these steps are necessary but not sufficient for establishing fitness for intended
use, but they are pre-condition as they establish the software documentation.

### CLI in CI/CD
You can use this CLI in 2 ways:

* via the docker image published on github (https://github.com/medical-software-quality/documentation-as-code/pkgs/container/quality).
* via the binaries [published as assets on github](https://github.com/medical-software-quality/documentation-as-code/releases)

The CLI is specified [here](./features/README.md) according to this repositories' specification.
