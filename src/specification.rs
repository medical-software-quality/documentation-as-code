use std::path::PathBuf;

use gherkin::{Feature, GherkinEnv};
use indexmap::{IndexMap, IndexSet};
use lazy_static::lazy_static;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use regex::Regex;
use serde::Serialize;

use super::files;

pub struct Error(pub Vec<String>);

#[derive(Debug, Clone, Copy)]
pub enum Spec {
    Requirements,
    Design,
    Risks,
    Tests,
    Manual,
    Retire,
}

impl Spec {
    pub fn file_name(&self) -> &'static str {
        match self {
            Spec::Requirements => unreachable!(),
            Spec::Design => "design_specification.md",
            Spec::Risks => "risk_assessment.md",
            Spec::Tests => "verification_plan.md",
            Spec::Manual => "user_manual.md",
            Spec::Retire => "retirement_plan.md",
        }
    }
}

pub type Trace = IndexMap<String, IndexSet<String>>;
pub type Requirements = IndexMap<String, String>;

#[derive(Debug, Serialize, Default)]
pub struct Document {
    text: String, // markdown
    trace: Trace,
}

impl Document {
    pub fn try_new(text: String, spec: Spec) -> Result<Self, Error> {
        get_trace(&text, spec).map(|trace| Self { text, trace })
    }
}

#[derive(Debug, Serialize)]
pub struct Documents {
    requirements: Requirements,
    design_specification: Document,
    risk_assessment: Document,
    verification_plan: Document,
    user_manual: Document,
    retirement_plan: Document,
}

impl Documents {
    pub fn try_new(
        requirements: Requirements,
        design_specification: Document,
        risk_assessment: Document,
        verification_plan: Document,
        user_manual: Document,
        retirement_plan: Document,
    ) -> Result<Self, Error> {
        check_documentation(
            &requirements,
            &verification_plan,
            &risk_assessment,
            &design_specification,
            &user_manual,
            &retirement_plan,
        )?;
        Ok(Self {
            requirements,
            design_specification,
            risk_assessment,
            verification_plan,
            user_manual,
            retirement_plan,
        })
    }
}

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

fn parse(markdown_input: &str, spec: Spec) -> (Trace, Vec<String>) {
    let expected_title = match spec {
        Spec::Design => "Design specification",
        Spec::Tests => "Verification plan",
        Spec::Risks => "Risk assessment",
        Spec::Manual => "User manual",
        Spec::Retire => "Retirement plan",
        Spec::Requirements => unreachable!(),
    };

    let parser = Parser::new(markdown_input);

    let mut in_heading = false;
    let mut in_title = false;
    let mut has_title = false;
    let mut trace_state = TraceState::None;
    let mut errors = vec![];
    let mut trace = Trace::new();
    parser.for_each(|event| match event {
        Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
            in_title = true;
            if has_title {
                errors.push(format!(
                    "\"{}\" must contain a single title (#) with \"# {expected_title}\" but it contains at least two titles.",
                    spec.file_name(),
                ))
            }
            has_title = true;
        }
        Event::Text(inner) if in_title => {
            if inner.as_bytes() != expected_title.as_bytes() {
                errors.push(format!(
                    "\"{}\" must start with \"# {expected_title}\" but starts with \"# {inner}\"",
                    spec.file_name(),
                ))
            }
        }
        Event::End(Tag::Heading(HeadingLevel::H1, _, _)) => {
            in_title = false;
        }
        Event::Start(Tag::Heading(HeadingLevel::H2, _, _)) => {
            in_heading = true;
        }
        Event::Text(inner) if in_heading => {
            let id = extract_identifier(inner.as_ref());
            if let Some(id) = id {
                if trace.insert(id.to_string(), Default::default()).is_some() {
                    errors.push(format!("\"{}\" must contain unique identifiers, but \"{id}\" is not", spec.file_name()))
                }
            } else {
                errors.push(format!("\"{}\" must contain sections of the form \"## ID - title\", but \"{inner}\" is not in this form", spec.file_name()))
            }
        }
        Event::End(Tag::Heading(HeadingLevel::H2, _, _)) => {
            in_heading = false;
        }
        Event::Start(Tag::Heading(HeadingLevel::H3, _, _)) if !trace.is_empty() => {
            trace_state = TraceState::CheckHeading;
        }
        Event::Text(inner)
            if trace_state == TraceState::CheckHeading && inner.as_ref() == "Trace" =>
        {
            trace_state = TraceState::Heading;
        }
        Event::End(Tag::Heading(HeadingLevel::H3, _, _)) if trace_state == TraceState::Heading => {
            trace_state = TraceState::Body;
        }
        // if the heading is not trace, revert to no state
        Event::End(Tag::Heading(HeadingLevel::H3, _, _))
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
            trace.last_mut().unwrap().1.insert(inner.to_string());
        }
        Event::End(Tag::Item) if trace_state == TraceState::Item => {
            trace_state = TraceState::List;
        }
        _ => {}
    });
    if !has_title {
        errors.push(format!(
            "\"{}\" must start with \"# {expected_title}\", but the document has no title",
            spec.file_name(),
        ))
    }

    (trace, errors)
}

fn check_ids<'a, I: Iterator<Item = &'a String> + Clone>(headings: I, spec: Spec) -> Vec<String> {
    let errors: Vec<String> = match spec {
        Spec::Requirements => headings
            .filter(|heading| !heading.starts_with("FS-"))
            .map(|heading| {
                format!("Headings in requirements must start with \"FS-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Design => headings
            .filter(|heading| !heading.starts_with("DS-"))
            .map(|heading| {
                format!("Headings in design specification must start with \"DS-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Risks => headings
            .filter(|heading| !heading.starts_with("RISK-"))
            .map(|heading| {
                format!("Headings in risk assessment must start with \"RISK-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Tests => headings
            .filter(|heading| !heading.starts_with("TEST-"))
            .map(|heading| {
                format!("Headings in verification plan must start with \"TEST-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Manual => headings
            .filter(|heading| !heading.starts_with("USER-"))
            .map(|heading| {
                format!("Headings in user manual must start with \"USER-\". \"{heading}\" does not.")
            })
            .collect(),
        Spec::Retire => headings
            .filter(|heading| !heading.starts_with("RETIRE-"))
            .map(|heading| {
                format!("Headings in retirement plan must start with \"RETIRE-\". \"{heading}\" does not.")
            })
            .collect(),
    };

    errors
}

fn check_trace(trace: &Trace) -> Vec<String> {
    let mut errors = vec![];

    for (key, values) in trace {
        for value in values {
            if trace.contains_key(value) {
                errors.push(format!(
                    "Trace of {key} cannot be to other items on the same document ({value} is)"
                ))
            }
        }
    }

    errors
}

fn get_trace(content: &str, spec: Spec) -> Result<Trace, Error> {
    let (trace, mut errors) = parse(content, spec);

    errors.extend(check_ids(trace.keys(), spec));
    errors.extend(check_trace(&trace));

    if errors.is_empty() {
        Ok(trace)
    } else {
        Err(Error(errors))
    }
}

fn check_documentation(
    requirements: &Requirements,
    verification_plan: &Document,
    risk_assessment: &Document,
    design_specification: &Document,
    user_manual: &Document,
    retirement_plan: &Document,
) -> Result<(), Error> {
    let mut errors = vec![];

    let tests = &verification_plan.trace;
    let risks = &risk_assessment.trace;
    let designs = &design_specification.trace;
    let requirements = &requirements;
    let user_manual = &user_manual.trace;
    let retirement_plan = &retirement_plan.trace;

    let mut uncovered_requirements = requirements.keys().collect::<IndexSet<_>>();
    for (test, values) in tests {
        for value in values {
            let is_valid = risks.contains_key(value) || requirements.contains_key(value);
            if !is_valid {
                let in_other = tests.contains_key(value) || designs.contains_key(value);
                if in_other {
                    errors.push(format!("Tests can only be traced to existing risks or requirements, but {test} is traced to a design or test"));
                } else {
                    errors.push(format!("Tests can only be traced to existing risks or requirements, but {test} is traced to something else"));
                }
            }
            uncovered_requirements.remove(value);
        }
    }
    if !uncovered_requirements.is_empty() {
        errors.push(format!(
            "All requirements must be covered by tests, but {uncovered_requirements:?} are not"
        ));
    }

    for (risk, values) in risks {
        for value in values {
            let is_valid = requirements.contains_key(value) || designs.contains_key(value);
            if !is_valid {
                let in_other = risks.contains_key(value) || tests.contains_key(value);
                if in_other {
                    errors.push(format!("Risks can only be traced to existing requirements or designs, but {risk} traces to a risk or test"));
                } else {
                    errors.push(format!("Risks can only be traced to existing requirements or designs, but {risk} traces to something else"));
                }
            }
        }
    }

    for (design, values) in designs {
        for value in values {
            let is_valid = requirements.contains_key(value);
            if !is_valid {
                let in_other = risks.contains_key(value)
                    || tests.contains_key(value)
                    || designs.contains_key(value);
                if in_other {
                    errors.push(format!("Designs can only be traced to existing requirements, but {design} is traced to a risk, test or another design"));
                } else {
                    errors.push(format!("Designs can only be traced to existing requirements, but {design} is traced to something else"));
                }
            }
        }
    }

    for (user, values) in user_manual {
        for value in values {
            let is_valid = requirements.contains_key(value);
            if !is_valid {
                let in_other = risks.contains_key(value)
                    || tests.contains_key(value)
                    || designs.contains_key(value);
                if in_other {
                    errors.push(format!("Users can only be traced to existing requirements, but {user} is traced to a risk, test or another design"));
                } else {
                    errors.push(format!("Users can only be traced to existing requirements, but {user} is traced to something else"));
                }
            }
        }
    }

    for (retire, values) in retirement_plan {
        if !values.is_empty() {
            errors.push(format!(
                "Retirement plan cannot be traced, but {retire} is traced to something else"
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error(errors))
    }
}

pub fn get_specification(project: PathBuf, errors: &mut Vec<String>) -> IndexMap<String, String> {
    let path = project.join("features");

    let paths = match files::list_directory(path) {
        Ok(paths) => paths,
        Err(error) => {
            errors.push(error);
            return Default::default();
        }
    };

    let mut headings = IndexMap::new();
    paths
        .into_iter()
        // get Gherkin feature files
        .filter(|path| path.extension().unwrap_or_default() == "feature")
        .for_each(|path| {
            // open the file
            let content = match files::read_file(&path) {
                Ok(content) => content,
                Err(error) => {
                    errors.push(error);
                    return;
                }
            };

            // parse it as a Gherkin feature
            let feature = match Feature::parse(&content, GherkinEnv::default()) {
                Ok(feature) => feature,
                Err(error) => {
                    errors.push(error.to_string());
                    return;
                }
            };

            let id = if let Some(id) = extract_identifier(&feature.name) {
                id
            } else {
                errors.push(
                    format!("Every feature's title must be of the form \"FS-<id> - <title>\", but {} in {} does not.", feature.name, path.display()),
                );
                return;
            };

            if headings.contains_key(id) {
                errors.push(format!("Headings must be unique, but {id} is not"))
            } else {
                headings.insert(id.to_string(), content);
            }
        });

    errors.extend(check_ids(headings.keys(), Spec::Requirements));

    headings
}
