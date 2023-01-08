# DS-1 - Rust programming language

This CLI is written in Rust programming language. This language allows for a small memory footprint
and compatibility across most operating systems. It is also strongly typed, allowing for 
a small maintainance and reduced risks of semantic errors.

# DS-2 - CLI

The interface of this application is a command line interface. This catters for the primary
audience of this application, developers.

# DS-3 - Docker image

This application is also packaged as a docker image. This provides a convenient installation
process for developers. The image is based off Linux alpine to reduce the docker image size.

# DS-4 - Gherkin

This application uses Gherkin to declare its specification and tests. This provides a convenient
language to describe the application's behavior. This catters for its secondary
audience, QA / validation leads without a software background.

# DS-5 - Documentation on the file system

This application assumes that the documentation is in the host's file system (as opposed to 
e.g. an external git repository). This assumption reduces the complexity of this application
at the cost of less flexbility (users must download the documentation to the host).
