use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

mod files;
mod specification;

use specification::*;

fn get_document(
    project: PathBuf,
    spec: Spec,
    file_name: &str,
    errors: &mut Vec<String>,
) -> Option<Document> {
    let path = project.join(file_name);

    let content = match files::read_file(path) {
        Ok(content) => content,
        Err(error) => {
            errors.push(error);
            return None;
        }
    };
    match Document::try_new(content, spec) {
        Ok(document) => Some(document),
        Err(Error(new_errors)) => {
            errors.extend(new_errors);
            None
        }
    }
}

fn get_documents(project: PathBuf) -> Result<Documents, Error> {
    let mut errors = vec![];

    let requirements = get_specification(project.clone(), &mut errors);

    let design = get_document(
        project.clone(),
        Spec::Design,
        "design_specification.md",
        &mut errors,
    );

    let risk_assessment = get_document(
        project.clone(),
        Spec::Risks,
        "risk_assessment.md",
        &mut errors,
    );

    let verification_plan = get_document(project, Spec::Tests, "verification_plan.md", &mut errors);

    if errors.is_empty() {
        Documents::try_new(
            requirements,
            design.unwrap(),
            risk_assessment.unwrap(),
            verification_plan.unwrap(),
        )
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

    let documents = match get_documents(project) {
        Ok(documents) => documents,
        Err(Error(errors)) => {
            for error in errors {
                println!("ERROR: {error}");
            }
            return ExitCode::FAILURE;
        }
    };

    let result = serde_json::to_string(&documents).unwrap();

    println!("{result}");

    ExitCode::SUCCESS
}
