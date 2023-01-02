use cucumber::{given, then, when, World as _};

#[derive(cucumber::World, Debug, Default)]
struct World {
    path: String,
}

#[given(expr = "a solution")]
fn someone_is_hungry(w: &mut World) {
    w.user = 
}

#[when(regex = r"^(?:he|she|they) eats? (\d+) cucumbers?$")]
fn eat_cucumbers(w: &mut World, count: usize) {
    w.capacity += count;

    assert!(w.capacity < 4, "{} exploded!", w.user.as_ref().unwrap());
}

#[then("she is full")]
fn is_full(w: &mut World) {
    assert_eq!(w.capacity, 3, "{} isn't full!", w.user.as_ref().unwrap());
}

fn main() {
    futures::executor::block_on(World::run("documentation/features"));
}
