use world::{World, Cell};

mod world;

fn main() {
    let mut world = World::new(10, 10);

    world[(2, 1)] = Cell::Live;
    world[(3, 2)] = Cell::Live;
    world[(1, 3)] = Cell::Live;
    world[(2, 3)] = Cell::Live;
    world[(3, 3)] = Cell::Live;

    for _ in 0..20 { world.cycle(); }

    println!("{world:?}");
}

