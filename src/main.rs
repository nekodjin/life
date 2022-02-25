use world::{World, LIVE, DEAD};

mod world;

fn main() {
    let mut world = World::new(8, 6);

    world[( 0,  0)] = LIVE;
    world[(-1, -1)] = LIVE;

    println!("{world:?}");
}

