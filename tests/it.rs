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
    if !design.is_empty() {
        std::fs::write(dir.join("design_specification.md"), design).unwrap();
    } else {
        std::fs::write(
            dir.join("design_specification.md"),
            "# Design specification",
        )
        .unwrap();
    }
    if !risk.is_empty() {
        std::fs::write(dir.join("risk_assessment.md"), risk).unwrap();
    } else {
        std::fs::write(dir.join("risk_assessment.md"), "# Risk assessment").unwrap();
    }
    if !test.is_empty() {
        std::fs::write(dir.join("verification_plan.md"), test).unwrap();
    } else {
        std::fs::write(dir.join("verification_plan.md"), "# Verification plan").unwrap();
    }
    dir
}

#[derive(cucumber::World, Debug, Default)]
struct World {
    feature: String,
    risk_assessment: String,
    verification_plan: String,
    design_specification: String,
    has_spec: bool,
    command: Option<Command>,
}

#[given(expr = "software without a specification")]
#[given(expr = "software without any documentation")]
fn software_unspecified(w: &mut World) {
    w.has_spec = false;
}

#[given(expr = "the following feature")]
fn a_feature(w: &mut World, step: &Step) {
    w.feature = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[given(expr = "the following content in `design_specification.md`")]
fn a_design(w: &mut World, step: &Step) {
    w.design_specification = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[given(expr = "the following content in `risk_assessment.md`")]
#[given(expr = "the following valid risk assessment")]
fn a_risk(w: &mut World, step: &Step) {
    w.risk_assessment = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[given(expr = "the following content in `verification_plan.md`")]
fn a_test(w: &mut World, step: &Step) {
    w.verification_plan = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[when(expr = "we check its documentation")]
#[when(expr = "we check it")]
fn check_docs(w: &mut World) {
    let path = if w.has_spec {
        create_local_project(
            &w.feature,
            &w.design_specification,
            &w.risk_assessment,
            &w.verification_plan,
        )
    } else {
        "./not_a_directory".into()
    };

    let mut cmd = Command::cargo_bin("quality").unwrap();

    cmd.arg("--path").arg(&path);

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

#[then("we get the following JSON")]
fn check_json(w: &mut World, step: &Step) {
    let command = std::mem::take(&mut w.command);
    let assert = command.unwrap().assert().success();
    let output = assert.get_output();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),
        serde_json::from_str::<serde_json::Value>(step.docstring.as_ref().unwrap()).unwrap()
    );
}

fn main() {
    futures::executor::block_on(World::run("documentation/features"));
}