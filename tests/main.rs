use assert_cmd::Command;
use cucumber::{given, then, when, World as _};
use gherkin::Step;
use predicates::prelude::PredicateBooleanExt;

fn create_local_project(spec: &str, risk: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir();
    let dir = dir.join("project");
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::create_dir(&dir);
    let _ = std::fs::create_dir(dir.join("features"));
    if !spec.is_empty() {
        let path = dir.join("features").join("some.feature");
        std::fs::write(path, spec).unwrap();
    }
    std::fs::write(dir.join("design_specification.md"), "").unwrap();
    std::fs::write(dir.join("risk_assessment.md"), risk).unwrap();
    std::fs::write(dir.join("test_plan.md"), "").unwrap();
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

#[given(expr = "a solution without any documentation")]
fn an_invalid_documentation(w: &mut World) {
    w.path = "./tests/fixtures/not_a_directory".into();
}

#[given(expr = "the following feature")]
fn a_feature(w: &mut World, step: &Step) {
    w.path = create_local_project(step.docstring.as_ref().unwrap(), "");
}

#[given(expr = "the following risk assessment in `risk_assessment.md`")]
#[given(expr = "the following valid risk assessment")]
fn a_risk(w: &mut World, step: &Step) {
    w.path = create_local_project("", step.docstring.as_ref().unwrap());
}

#[given("the following invalid specification")]
fn invalid_specification(w: &mut World) {
    w.path = "./tests/fixtures/not_a_directory".into();
}

#[given(expr = "sofware correctly specified")]
#[given(expr = "a solution with a valid risk assessment")]
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
fn check_fails(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command.unwrap().assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("risk_assessment.md")),
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
                "Headings in risk assessment must start with \"RISK-\". \"Risk 1\" does not.",
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

#[then("we get an error regarding a wrong trace")]
fn check_fails_identifier_trace(w: &mut World) {
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
