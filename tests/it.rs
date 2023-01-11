use assert_cmd::Command;
use cucumber::{given, then, when, World as _};
use gherkin::Step;
use predicates::prelude::PredicateBooleanExt;

fn create_local_project(spec: &str, design: &str, risk: &str, test: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir();
    let dir = dir.join("project");
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::create_dir(&dir);
    let _ = std::fs::create_dir(dir.join("features"));
    if !spec.is_empty() {
        let path = dir.join("features").join("some.feature");
        std::fs::write(path, spec).unwrap();
    }
    std::fs::write(dir.join("design_specification.md"), design).unwrap();
    std::fs::write(dir.join("risk_assessment.md"), risk).unwrap();
    std::fs::write(dir.join("verification_plan.md"), test).unwrap();
    dir
}

#[derive(cucumber::World, Debug, Default)]
struct World {
    path: std::path::PathBuf,
    command: Option<Command>,
}

#[given(expr = "software without a specification")]
fn software_unspecified(w: &mut World) {
    w.path = "./tests/fixtures/not_a_directory".into();
}

#[given(expr = "software without any documentation")]
fn an_invalid_documentation(w: &mut World) {
    w.path = "./tests/fixtures/not_a_directory".into();
}

#[given(expr = "the following feature")]
fn a_feature(w: &mut World, step: &Step) {
    w.path = create_local_project(
        step.docstring.as_ref().unwrap(),
        "# Design specification",
        "# Risk assessment",
        "# Verification plan",
    );
}

#[given(expr = "the following content in `design_specification.md`")]
fn a_design(w: &mut World, step: &Step) {
    w.path = create_local_project(
        "",
        step.docstring.as_ref().unwrap(),
        "# Risk assessment",
        "# Verification plan",
    );
}

#[given(expr = "the following content in `risk_assessment.md`")]
#[given(expr = "the following valid risk assessment")]
fn a_risk(w: &mut World, step: &Step) {
    w.path = create_local_project(
        "",
        "# Design specification",
        step.docstring.as_ref().unwrap(),
        "# Verification plan",
    );
}

#[given(expr = "the following content in `verification_plan.md`")]
fn a_test(w: &mut World, step: &Step) {
    w.path = create_local_project(
        "",
        "# Design specification",
        "# Risk assessment",
        step.docstring.as_ref().unwrap(),
    );
}

#[given("the following invalid specification")]
fn invalid_specification(w: &mut World) {
    w.path = "./tests/fixtures/not_a_directory".into();
}

#[given(expr = "sofware correctly specified")]
#[given(expr = "software with a valid risk assessment")]
fn a_valid_documentation(w: &mut World) {
    w.path = "./tests/fixtures/valid_project".into();
}

#[when(expr = "we check its documentation")]
#[when(expr = "we check it")]
fn check_docs(w: &mut World) {
    let mut cmd = Command::cargo_bin("quality").unwrap();

    cmd.arg("--path").arg(&w.path);

    w.command = Some(cmd);
}

#[then("we get an error of a missing risk assessment file")]
fn missing_risk(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command.unwrap().assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("risk_assessment.md")),
    );
}

#[then("we get an error of a missing design specification")]
fn missing_design(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command.unwrap().assert().failure().stdout(
        predicates::str::contains("ERROR")
            .and(predicates::str::contains("design_specification.md")),
    );
}

#[then("we get an error of a missing verification plan")]
fn missing_verification(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command.unwrap().assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("verification_plan.md")),
    );
}

#[then("we get an error of an incorrect risk assessment")]
fn check_fails_identifier_risk(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Headings in risk assessment must start with \"RISK-\".",
            )),
        );
}

#[then("we get an error of an incorrect design specification")]
fn check_fails_identifier_design(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Headings in design specification must start with \"DS-\".",
            )),
        );
}

#[then("we get an error of an incorrect header in design specification")]
fn then_missing_header_in_design(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "The document must start with \"# Design specification\" but starts with \"# Design statement\"",
            )),
        );
}

#[then("we get an error of an incorrect verification plan")]
fn check_fails_identifier_verification(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Headings in verification plan must start with \"TEST-\".",
            )),
        );
}

#[then("we get an error of a missing specification")]
fn check_fails_specification(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(predicates::str::contains("/features"));
}

#[then("we get an error regarding a wrong identifier")]
fn check_fails_identifier(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "must contain a heading with a valid identifier followed by a title",
            )),
        );
}

#[then("we get an error regarding a wrong trace in risks")]
fn check_fails_identifier_trace_risk(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Risks can only be traced to existing requirements or designs, but RISK-1 traces to something else",
            )),
        );
}

#[then("we get an error regarding a wrong trace in design")]
fn check_fails_identifier_trace_design(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Designs can only be traced to existing requirements, but DS-1 is traced to something else",
            )),
        );
}

#[then("we get an error regarding a wrong trace in verification plan")]
fn check_fails_identifier_trace_verification(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Tests can only be traced to existing risks or requirements, but TEST-1 is traced to something else",
            )),
        );
}

#[then("we get an error regarding wrong Gherkin")]
fn check_fails_gherkin(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(predicates::str::contains("ERROR"));
}

#[then("we get no error")]
#[then("we get no error regarding its specification")]
fn check_ok(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command.unwrap().assert().success();
}

fn main() {
    futures::executor::block_on(World::run("documentation/features"));
}
