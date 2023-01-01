use pulldown_cmark::{html, Event, HeadingLevel, Parser, Tag};

use crate::{Documentation, Spec};

fn one_down(level: HeadingLevel) -> HeadingLevel {
    match level {
        HeadingLevel::H1 => HeadingLevel::H2,
        HeadingLevel::H2 => HeadingLevel::H3,
        HeadingLevel::H3 => HeadingLevel::H4,
        HeadingLevel::H4 => HeadingLevel::H5,
        HeadingLevel::H5 => HeadingLevel::H6,
        HeadingLevel::H6 => HeadingLevel::H6,
    }
}

fn heading_down(event: Event) -> Event {
    match event {
        Event::Start(Tag::Heading(l, a, b)) => Event::Start(Tag::Heading(one_down(l), a, b)),
        Event::End(Tag::Heading(l, a, b)) => Event::End(Tag::Heading(one_down(l), a, b)),
        _ => event,
    }
}

fn heading(spec: Spec) -> std::array::IntoIter<pulldown_cmark::Event<'static>, 3> {
    let header = match spec {
        Spec::Requirements => "User requirements specification",
        Spec::Design => "Design specification",
        Spec::Risks => "Risk assessment",
        Spec::Tests => "Test plan",
    };

    return [
        Event::Start(Tag::Heading(HeadingLevel::H1, None, vec![])),
        Event::Text(header.into()),
        Event::End(Tag::Heading(HeadingLevel::H1, None, vec![])),
    ]
    .into_iter();
}

pub fn to_html(documentation: Documentation) -> String {
    let requirements = Parser::new(&documentation.requirements.0);
    let requirements = requirements.map(heading_down);
    let requirements = heading(Spec::Requirements).chain(requirements);

    let design = Parser::new(&documentation.design.0);
    let design = design.map(heading_down);
    let design = heading(Spec::Design).chain(design);

    let risks = Parser::new(&documentation.risks.0);
    let risks = risks.map(heading_down);
    let risks = heading(Spec::Risks).chain(risks);

    let tests = Parser::new(&documentation.tests.0);
    let tests = tests.map(heading_down);
    let tests = heading(Spec::Tests).chain(tests);

    let document = requirements.chain(design).chain(risks).chain(tests);

    let mut html_output = String::new();
    html::push_html(&mut html_output, document);
    html_output
}
