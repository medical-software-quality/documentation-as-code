use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use cucumber::{given, then, when, World as _};
use gherkin::Step;
use predicates::prelude::PredicateBooleanExt;

fn dir_name<'a>(iter: impl Iterator<Item = &'a str>) -> String {
    let mut hasher = DefaultHasher::new();
    iter.for_each(|item| item.hash(&mut hasher));
    format!("{}", hasher.finish())
}

fn create_local_project(
    spec: &str,
    design: &str,
    risk: &str,
    test: &str,
    user_manual: &str,
    operator_manual: &str,
    retirement_plan: &str,
) -> PathBuf {
    let dir = std::env::temp_dir();
    let dir = dir.join(dir_name(
        [
            spec,
            design,
            risk,
            test,
            user_manual,
            operator_manual,
            retirement_plan,
        ]
        .into_iter(),
    ));
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
    if !user_manual.is_empty() {
        std::fs::write(dir.join("user_manual.md"), user_manual).unwrap();
    } else {
        std::fs::write(dir.join("user_manual.md"), "# User manual").unwrap();
    }
    if !operator_manual.is_empty() {
        std::fs::write(dir.join("operator_manual.md"), operator_manual).unwrap();
    } else {
        std::fs::write(dir.join("operator_manual.md"), "# Operator manual").unwrap();
    }
    if !retirement_plan.is_empty() {
        std::fs::write(dir.join("retirement_plan.md"), retirement_plan).unwrap();
    } else {
        std::fs::write(dir.join("retirement_plan.md"), "# Retirement plan").unwrap();
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

fn command(path: &Path) -> Command {
    let mut cmd = Command::cargo_bin("quality").unwrap();
    cmd.arg("--path").arg(path);
    cmd
}

#[derive(cucumber::World, Debug, Default)]
struct World {
    feature: String,
    risk_assessment: String,
    verification_plan: String,
    design_specification: String,
    user_manual: String,
    operator_manual: String,
    retirement_plan: String,
    has_spec: bool,
    path: PathBuf,
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
fn a_risk(w: &mut World, step: &Step) {
    w.risk_assessment = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[given(expr = "the following content in `verification_plan.md`")]
fn a_test(w: &mut World, step: &Step) {
    w.verification_plan = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[given(expr = "the following content in `user_manual.md`")]
fn a_user_manual(w: &mut World, step: &Step) {
    w.user_manual = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[given(expr = "the following content in `operator_manual.md`")]
fn a_operator_manual(w: &mut World, step: &Step) {
    w.operator_manual = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[given(expr = "the following content in `retirement_plan.md`")]
fn a_retirement(w: &mut World, step: &Step) {
    w.retirement_plan = step.docstring.as_ref().unwrap().clone();
    w.has_spec = true;
}

#[when(expr = "we check its documentation")]
#[when(expr = "we check it")]
fn check_docs(w: &mut World) {
    w.path = if w.has_spec {
        create_local_project(
            &w.feature,
            &w.design_specification,
            &w.risk_assessment,
            &w.verification_plan,
            &w.user_manual,
            &w.operator_manual,
            &w.retirement_plan,
        )
    } else {
        "./not_a_directory".into()
    };
}

#[then("we get an error of a missing risk assessment file")]
fn missing_risk(w: &mut World) {
    command(&w.path).assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("risk_assessment.md")),
    );
}

#[then("we get an error of a missing design specification")]
fn missing_design(w: &mut World) {
    command(&w.path).assert().failure().stdout(
        predicates::str::contains("ERROR")
            .and(predicates::str::contains("design_specification.md")),
    );
}

#[then("we get an error of a missing retirement plan")]
fn missing_retirement(w: &mut World) {
    command(&w.path).assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("retirement_plan.md")),
    );
}

#[then("we get an error of a missing verification plan")]
fn missing_verification(w: &mut World) {
    command(&w.path).assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("verification_plan.md")),
    );
}

#[then("we get an error of a missing user manual file")]
fn missing_user_manual(w: &mut World) {
    command(&w.path).assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("user_manual.md")),
    );
}

#[then("we get an error of a missing operator manual file")]
fn missing_operator_manual(w: &mut World) {
    command(&w.path).assert().failure().stdout(
        predicates::str::contains("ERROR").and(predicates::str::contains("operator_manual.md")),
    );
}

#[then("we get an error of an incorrect risk assessment")]
fn check_fails_identifier_risk(w: &mut World) {
    command(&w.path)
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
    command(&w.path)
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
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "\"design_specification.md\" must start with \"# Design specification\" but starts with \"# Design statement\"",
            )),
        );
}

#[then("we get an error of an incorrect verification plan")]
fn check_fails_identifier_verification(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Headings in verification plan must start with \"TEST-\".",
            )),
        );
}

#[then("we get an error of an incorrect user manual")]
fn check_fails_identifier_user_manual(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Headings in user manual must start with \"USER-\".",
            )),
        );
}

#[then("we get an error of an incorrect operator manual")]
fn check_fails_identifier_operator_manual(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Headings in operator manual must start with \"OPERATOR-\".",
            )),
        );
}

#[then("we get an error of an incorrect retirement plan")]
fn check_fails_identifier_retirement_plan(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Headings in retirement plan must start with \"RETIRE-\".",
            )),
        );
}

#[then("we get an error of a missing specification")]
fn check_fails_specification(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(predicates::str::contains("/features"));
}

#[then("we get an error regarding a wrong identifier")]
fn check_fails_identifier(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Every feature's title must be of the form \"FS-<id> - <title>\"",
            )),
        );
}

#[then("we get an error regarding a wrong trace in risks")]
fn check_fails_identifier_trace_risk(w: &mut World) {
    command(&w.path)
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
    command(&w.path)
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
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Tests can only be traced to existing risks or requirements, but TEST-1 is traced to something else",
            )),
        );
}

#[then("we get an error regarding a wrong trace in user manual")]
fn check_fails_identifier_trace_manual(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(
            predicates::str::contains("ERROR").and(predicates::str::contains(
                "Users can only be traced to existing requirements, but USER-1 is traced to something else",
            )),
        );
}

#[then("we get an error regarding wrong Gherkin")]
fn check_fails_gherkin(w: &mut World) {
    command(&w.path)
        .assert()
        .failure()
        .stdout(predicates::str::contains("ERROR"));
}

#[then("we get no error")]
fn check_ok(w: &mut World) {
    command(&w.path).assert().success();
}

#[then("we get the following JSON")]
fn check_json(w: &mut World, step: &Step) {
    let assert = command(&w.path).assert().success();
    let output = assert.get_output();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),
        serde_json::from_str::<serde_json::Value>(step.docstring.as_ref().unwrap()).unwrap()
    );
}

fn main() {
    futures::executor::block_on(World::run("documentation/features"));
}
