use std::process::ExitCode;
use std::{collections::HashSet, path::PathBuf};

use either::Either;
use lazy_static::lazy_static;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use regex::Regex;

mod files;
mod to_html;

#[derive(Debug, Clone, Copy)]
enum Spec {
    Requirements,
    Design,
    Risks,
}

#[derive(Debug)]
struct Documentation {
    pub requirements: Headings,
    pub design: Headings,
    pub risks: Headings,
}

#[derive(Debug)]
struct Headings(pub Vec<String>);

fn extract_identifier(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<id>.*?) - .*$").unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("id").map(|login| login.as_str()))
}

fn parse_headings(markdown_input: &str) -> Either<Headings, Vec<String>> {
    let parser = Parser::new(markdown_input);

    let mut in_heading = false;
    let mut headings = vec![];
    let mut errors = vec![];
    parser.for_each(|event| match event {
        Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
            in_heading = true;
        }
        Event::Text(inner) => {
            if in_heading {
                let id = extract_identifier(inner.as_ref());
                if let Some(id) = id {
                    headings.push(id.to_string())
                } else {
                    errors.push(format!("Can't parse the identifier of {}", inner.as_ref()))
                }
            }
        }
        Event::End(Tag::Heading(HeadingLevel::H1, _, _)) => {
            in_heading = false;
        }
        _ => {}
    });
    if !errors.is_empty() {
        Either::Right(errors)
    } else {
        Either::Left(Headings(headings))
    }
}

fn check_ids(headings: Headings, spec: Spec) -> Either<Headings, Vec<String>> {
    let errors: Vec<String> = match spec {
        Spec::Requirements => headings
            .0
            .iter()
            .filter(|heading| !heading.starts_with("URS-"))
            .map(|heading| {
                format!("Headings in URS must start with \"URS-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Design => headings
            .0
            .iter()
            .filter(|heading| !heading.starts_with("DS-"))
            .map(|heading| {
                format!("Headings in Design must start with \"DS-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Risks => headings
            .0
            .iter()
            .filter(|heading| !heading.starts_with("RISK-"))
            .map(|heading| {
                format!("Headings in Risk assessment must start with \"RISK-\". \"{heading}\" does not.")
            })
            .collect(),
    };

    if !errors.is_empty() {
        Either::Right(errors)
    } else {
        Either::Left(headings)
    }
}

fn check_uniqueness(headings: Headings) -> Either<Headings, String> {
    let unique = headings.0.iter().collect::<HashSet<&String>>();

    if unique.len() != headings.0.len() {
        Either::Right(format!(
            "Headings in a document must have unique ids. However, {{{}}}",
            unique.iter().fold(",".to_string(), |mut acc, v| {
                acc.push_str(v);
                acc
            }),
        ))
    } else {
        Either::Left(headings)
    }
}

fn check_spec(content: &str, spec: Spec, errors: &mut Vec<String>) -> Option<Headings> {
    let headings = match parse_headings(content) {
        Either::Right(document_errors) => {
            errors.extend(document_errors);
            return None;
        }
        Either::Left(headings) => headings,
    };

    let headings = match check_ids(headings, spec) {
        Either::Right(document_errors) => {
            errors.extend(document_errors);
            return None;
        }
        Either::Left(headings) => headings,
    };

    let headings = match check_uniqueness(headings) {
        Either::Right(error) => {
            errors.push(error);
            return None;
        }
        Either::Left(headings) => headings,
    };

    Some(headings)
}

fn get_spec(
    project: PathBuf,
    spec: Spec,
    file_name: &str,
    errors: &mut Vec<String>,
) -> Option<(String, Headings)> {
    let path = project.join(file_name);

    let content = match files::read_file(path) {
        Ok(content) => content,
        Err(error) => {
            errors.push(error);
            return None;
        }
    };

    let headings = check_spec(&content, spec, errors)?;

    Some((content, headings))
}

fn main() -> ExitCode {
    let project = PathBuf::from("./tests/project1/documentation");

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

    let risks = get_spec(project, Spec::Risks, "risk_assessment.md", &mut errors);

    let (requirements, design, risks) = if errors.is_empty() {
        (requirements.unwrap(), design.unwrap(), risks.unwrap())
    } else {
        for error in errors {
            println!("ERROR: {error}");
        }
        return ExitCode::FAILURE;
    };

    /*Documentation {
        requirements,
        design,
        risks,
    };*/

    ExitCode::SUCCESS
}
