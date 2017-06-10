mod world;
mod color;
mod size;
mod entity;
mod point;

use world::*;
use color::*;
use std::collections::HashMap;

fn main() {
    let mut world = World::new();
    world.size.height = 50;
    world.size.width = 50;

    println!("{:?}", world);
}
