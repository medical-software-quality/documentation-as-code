use cucumber::{given, then, when, World as _};

use assert_cmd::Command;

#[derive(cucumber::World, Debug, Default)]
struct World {
    path: String,
    command: Option<Command>,
}

#[given(expr = "a solution without any documentation")]
fn an_invalid_documentation(w: &mut World) {
    w.path = "./tests/fixtures/not_a_directory".to_string();
}

#[given(expr = "a solution with a valid risk assessment")]
fn a_valid_documentation(w: &mut World) {
    w.path = "./tests/fixtures/project1".to_string();
}

#[when(expr = "we check its documentation")]
fn check_docs(w: &mut World) {
    let mut cmd = Command::cargo_bin("quality").unwrap();

    cmd.arg("--path").arg(&w.path);

    w.command = Some(cmd);
}

#[then("we get an error of a missing risk assessemnt file")]
fn check_fails(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(predicates::str::contains("risk_assessment.md"));
}

#[then("we get no error")]
fn check_ok(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(predicates::str::contains("risk_assessment.md"));
}

fn main() {
    futures::executor::block_on(World::run("documentation/features"));
}
