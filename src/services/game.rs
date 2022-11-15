use crate::entities::{game::Game, map::Map};

use super::{capacities, player};

pub fn create() -> Game {
    let capacities = capacities::create();
    let player = player::create(capacities);
    let map = Map::create();

    map.print();

    Game { player }
}
