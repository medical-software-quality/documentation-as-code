use cucumber::{given, then, when, World as _};

use assert_cmd::Command;

#[derive(cucumber::World, Debug, Default)]
struct World {
    path: String,
    command: Option<Command>,
}

#[given(expr = "a solution in a directory without any documentation")]
fn a_solution(w: &mut World) {
    w.path = "./tests/fixtures/not_a_directory".to_string();
}

#[when(expr = "its documentation is checked")]
fn check_docs(w: &mut World) {
    let mut cmd = Command::cargo_bin("quality").unwrap();

    cmd.arg("--path").arg(&w.path);

    w.command = Some(cmd);
}

#[then("the check fails")]
fn check_fails(w: &mut World) {
    let command = std::mem::take(&mut w.command);
    command
        .unwrap()
        .assert()
        .failure()
        .stdout(predicates::str::contains("ERROR"));
}

fn main() {
    futures::executor::block_on(World::run("documentation/features"));
}
