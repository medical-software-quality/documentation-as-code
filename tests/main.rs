use cucumber::{given, then, when, World as _};

use assert_cmd::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("quality")?;

    cmd.arg("path").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[derive(cucumber::World, Debug, Default)]
struct World {
    path: String,
}

#[given(expr = "a solution")]
fn a_solution(w: &mut World) {
    w.path = "./tests/project1".to_string();
}

#[when(expr = "its documentation is checked")]
fn check_docs(w: &mut World) {
    let mut cmd = Command::cargo_bin("quality").unwrap();

    cmd.arg("path").arg("test/file/doesnt/exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    assert!(w.capacity < 4, "{} exploded!", w.user.as_ref().unwrap());
}

#[then("she is full")]
fn is_full(w: &mut World) {
    assert_eq!(w.capacity, 3, "{} isn't full!", w.user.as_ref().unwrap());
}

fn main() {
    futures::executor::block_on(World::run("documentation/features"));
}
