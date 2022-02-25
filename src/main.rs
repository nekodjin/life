use world::{World, Cell};

mod world;

fn main() {
    let mut world = World::new(8, 6);

    world[(2, 1)] = Cell::Live;
    world[(2, 2)] = Cell::Live;
    world[(2, 3)] = Cell::Live;

    println!("{world:?}");

    world.cycle();

    println!("{world:?}");

    world.cycle();

    println!("{world:?}");
}

