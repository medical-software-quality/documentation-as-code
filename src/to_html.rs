use pulldown_cmark::{html, Event, HeadingLevel, Parser, Tag};

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

pub fn to_html(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);

    let parser = parser.map(heading_down);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
