use creature::Creature;
use rand::distributions::{Alphanumeric, DistString};
use std::collections::HashMap;
use world::World;

mod creature;
mod world;

static NUM_CREATURES: u32 = 1000;
static MAX_FIGHTS: u32 = 1000000;

fn main() {
    let creatures = create_creatures();
    let mut world = create_world(creatures);
    world.fight(MAX_FIGHTS);
}

fn create_world(creatures: Vec<Creature>) -> World {
    World {
        creatures,
        winners: HashMap::new(),
    }
}

fn create_creatures() -> Vec<Creature> {
    let mut creatures = Vec::new();
    for _ in 0..NUM_CREATURES {
        let name: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 5);
        creatures.push(Creature { name })
    }
    creatures
}
