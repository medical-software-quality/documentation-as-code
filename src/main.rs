use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

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

    let requirements = get_specification(project.clone(), &mut errors);

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

    let tests = get_spec(project, Spec::Tests, "verification_plan.md", &mut errors);

    let documentation = if errors.is_empty() {
        Documentation {
            requirements,
            design: design.unwrap(),
            risks: risks.unwrap(),
            tests: tests.unwrap(),
        }
    } else {
        return Err(Error(errors));
    };

    let errors = check_documentation(&documentation);
    if errors.is_empty() {
        Ok(documentation)
    } else {
        Err(Error(errors))
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let project = PathBuf::from(args.path);

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

    let result = to_html::to_html(documentation);

    println!("{result}");

    ExitCode::SUCCESS
}
