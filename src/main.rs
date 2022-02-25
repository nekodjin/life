use world::{World, Cell};

mod world;

fn main() {
    let mut world = World::new(8, 6);

    world[( 0,  0)] = Cell::Live;
    world[(-1, -1)] = Cell::Live;

    println!("{world:?}");
}

