use std::path::PathBuf;
use std::process::ExitCode;

mod files;
mod specification;
mod to_html;

use specification::*;

struct Error(Vec<String>);

fn get_spec(
    project: PathBuf,
    spec: Spec,
    file_name: &str,
    errors: &mut Vec<String>,
) -> Option<(String, Headings, Trace)> {
    let path = project.join(file_name);

    let content = match files::read_file(path) {
        Ok(content) => content,
        Err(error) => {
            errors.push(error);
            return None;
        }
    };

    let (headings, trace) = check_spec(&content, spec, errors)?;

    Some((content, headings, trace))
}

fn get_documentation(project: PathBuf) -> Result<Documentation, Error> {
    let mut errors = vec![];

    let requirements = get_spec(
        project.clone(),
        Spec::Requirements,
        "user_requirements_specification.md",
        &mut errors,
    );

    let design = get_spec(
        project.clone(),
        Spec::Design,
        "design_specification.md",
        &mut errors,
    );

    let risks = get_spec(
        project.clone(),
        Spec::Risks,
        "risk_assessment.md",
        &mut errors,
    );

    let tests = get_spec(project, Spec::Tests, "test_plan.md", &mut errors);

    if errors.is_empty() {
        Ok(Documentation {
            requirements: requirements.unwrap(),
            design: design.unwrap(),
            risks: risks.unwrap(),
            tests: tests.unwrap(),
        })
    } else {
        Err(Error(errors))
    }
}

fn main() -> ExitCode {
    let project = PathBuf::from("./tests/project1/documentation");

    let maybe_documentation = get_documentation(project);
    let documentation = match maybe_documentation {
        Ok(documentation) => documentation,
        Err(Error(errors)) => {
            for error in errors {
                println!("ERROR: {error}");
            }
            return ExitCode::FAILURE;
        }
    };

    let errors = check_documentation(&documentation);
    if !errors.is_empty() {
        for error in errors {
            println!("ERROR: {error}");
        }
        return ExitCode::FAILURE;
    };

    let result = to_html::to_html(documentation);

    println!("{result}");

    ExitCode::SUCCESS
}
