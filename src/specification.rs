use std::collections::HashSet;
use std::path::PathBuf;

use either::Either;
use gherkin::{Feature, GherkinEnv};
use lazy_static::lazy_static;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use regex::Regex;

use super::files;

#[derive(Debug, Clone, Copy)]
pub enum Spec {
    Requirements,
    Design,
    Risks,
    Tests,
}

#[derive(Debug)]
pub struct Documentation {
    pub requirements: (Vec<String>, Headings),
    pub design: (String, Headings, Trace),
    pub risks: (String, Headings, Trace),
    pub tests: (String, Headings, Trace),
}

pub type Headings = HashSet<String>;

pub type Trace = HashSet<(String, String)>;

fn extract_identifier(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<id>.*?) - .*$").unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("id").map(|login| login.as_str()))
}

#[derive(PartialEq, Eq)]
enum TraceState {
    None,
    CheckHeading,
    Heading,
    Body,
    List,
    Item,
}

fn parse_headings(markdown_input: &str) -> Either<(Headings, Trace), Vec<String>> {
    let parser = Parser::new(markdown_input);

    let mut in_heading = false;
    let mut trace_state = TraceState::None;
    let mut last_heading = None;
    let mut headings = Headings::new();
    let mut errors = vec![];
    let mut trace = Trace::new();
    parser.for_each(|event| match event {
        Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
            in_heading = true;
        }
        Event::Text(inner) if in_heading => {
            let id = extract_identifier(inner.as_ref());
            if let Some(id) = id {
                last_heading = Some(id.to_string());
                if !headings.insert(id.to_string()) {
                    errors.push(format!("Headings must be unique, but {id} is not"))
                }
            } else {
                errors.push(format!("Can't parse the identifier of {}", inner.as_ref()))
            }
        }
        Event::End(Tag::Heading(HeadingLevel::H1, _, _)) => {
            in_heading = false;
        }
        Event::Start(Tag::Heading(HeadingLevel::H2, _, _)) if !headings.is_empty() => {
            trace_state = TraceState::CheckHeading;
        }
        Event::Text(inner)
            if trace_state == TraceState::CheckHeading && inner.as_ref() == "Trace" =>
        {
            trace_state = TraceState::Heading;
        }
        Event::End(Tag::Heading(HeadingLevel::H2, _, _)) if trace_state == TraceState::Heading => {
            trace_state = TraceState::Body;
        }
        // if the heading is not trace, revert to no state
        Event::End(Tag::Heading(HeadingLevel::H2, _, _))
            if trace_state == TraceState::CheckHeading =>
        {
            trace_state = TraceState::None;
        }
        // list
        Event::Start(Tag::List(None)) if trace_state == TraceState::Body => {
            trace_state = TraceState::List;
        }
        Event::End(Tag::List(None)) if trace_state == TraceState::List => {
            trace_state = TraceState::Body;
        }
        // item
        Event::Start(Tag::Item) if trace_state == TraceState::List => {
            trace_state = TraceState::Item;
        }
        Event::Text(inner) if trace_state == TraceState::Item => {
            let heading = last_heading.clone().unwrap(); // guaranteed by !is_empty above
            trace.insert((heading, inner.to_string()));
        }
        Event::End(Tag::Item) if trace_state == TraceState::Item => {
            trace_state = TraceState::List;
        }
        _ => {}
    });
    if !errors.is_empty() {
        Either::Right(errors)
    } else {
        Either::Left((headings, trace))
    }
}

fn check_ids(headings: &Headings, spec: Spec) -> Vec<String> {
    let errors: Vec<String> = match spec {
        Spec::Requirements => headings
            .iter()
            .filter(|heading| !heading.starts_with("FS-"))
            .map(|heading| {
                format!("Headings in requirements must start with \"FS-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Design => headings
            .iter()
            .filter(|heading| !heading.starts_with("DS-"))
            .map(|heading| {
                format!("Headings in design specification must start with \"DS-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Risks => headings
            .iter()
            .filter(|heading| !heading.starts_with("RISK-"))
            .map(|heading| {
                format!("Headings in risk assessment must start with \"RISK-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Tests => headings
            .iter()
            .filter(|heading| !heading.starts_with("TEST-"))
            .map(|heading| {
                format!("Headings in verification plan must start with \"TEST-\". \"{heading}\" does not.")
            })
            .collect(),
    };

    errors
}

fn check_trace(trace: &Trace, headings: &Headings) -> Vec<String> {
    let mut errors = vec![];

    for (key, value) in trace {
        if headings.contains(value) {
            errors.push(format!("Trace of {key} cannot be to other identifiers on the same specification ({value} is)"))
        }
    }

    errors
}

pub fn check_spec(
    content: &str,
    spec: Spec,
    errors: &mut Vec<String>,
) -> Option<(Headings, Trace)> {
    let (headings, trace) = match parse_headings(content) {
        Either::Right(document_errors) => {
            errors.extend(document_errors);
            return None;
        }
        Either::Left((headings, trace)) => (headings, trace),
    };

    errors.extend(check_ids(&headings, spec));
    errors.extend(check_trace(&trace, &headings));

    Some((headings, trace))
}

pub fn check_documentation(document: &Documentation) -> Vec<String> {
    let mut errors = vec![];

    let tests_trace = &document.tests.2;
    let risks_trace = &document.risks.2;
    let design_trace = &document.design.2;
    let risks = &document.risks.1;
    let tests = &document.tests.1;
    let design = &document.design.1;
    let requirements = &document.requirements.1;

    for (test, value) in tests_trace {
        let is_valid = risks.contains(value) || requirements.contains(value);
        if !is_valid {
            let in_other = tests.contains(value) || design.contains(value);
            if in_other {
                errors.push(format!("Tests can only be traced to existing risks or requirements, but {test} is traced to a design or test"));
            } else {
                errors.push(format!("Tests can only be traced to existing risks or requirements, but {test} is traced to something else"));
            }
        }
    }

    for (risk, value) in risks_trace {
        let is_valid = requirements.contains(value) || design.contains(value);
        if !is_valid {
            let in_other = risks.contains(value) || tests.contains(value);
            if in_other {
                errors.push(format!("Risks can only be traced to existing requirements or designs, but {risk} traces to a risk or test"));
            } else {
                errors.push(format!("Risks can only be traced to existing requirements or designs, but {risk} traces to something else"));
            }
        }
    }

    for (design, value) in design_trace {
        let is_valid = requirements.contains(value);
        if !is_valid {
            let in_other = risks.contains(value) || tests.contains(value) || design.contains(value);
            if in_other {
                errors.push(format!("Designs can only be traced to existing requirements, but {design} is traced to a risk, test or another design"));
            } else {
                errors.push(format!("Designs can only be traced to existing requirements, but {design} is traced to something else"));
            }
        }
    }

    errors
}

pub fn get_specification(project: PathBuf, errors: &mut Vec<String>) -> (Vec<String>, Headings) {
    let path = project.join("features");

    let paths = match files::list_directory(path) {
        Ok(paths) => paths,
        Err(error) => {
            errors.push(error);
            return Default::default();
        }
    };

    let mut headings = HashSet::new();
    let paths = paths;

    let contents = paths
        .into_iter()
        // get Gherkin feature files
        .filter(|path| path.extension().unwrap_or_default() == "feature")
        // open the file
        .filter_map(|path| {
            let content = match files::read_file(path) {
                Ok(content) => Some(content),
                Err(error) => {
                    errors.push(error);
                    None
                }
            };

            // parse it as a Gherkin feature
            let content =
                content.and_then(
                    |content| match Feature::parse(&content, GherkinEnv::default()) {
                        Ok(feature) => Some((content, feature)),
                        Err(error) => {
                            errors.push(error.to_string());
                            None
                        }
                    },
                );

            // parse its header
            content.map(|(content, feature)| {
                let id = extract_identifier(&feature.name);
                if let Some(id) = id {
                    if !headings.insert(id.to_string()) {
                        errors.push(format!("Headings must be unique, but {id} is not"))
                    }
                } else {
                    errors.push(
                        format!("Every feature must contain a heading with a valid identifier followed by a title, but {:?} is not", feature
                        .name),
                    );
                }
                content
            })
        })
        .collect();

    errors.extend(check_ids(&headings, Spec::Requirements));

    (contents, headings)
}
