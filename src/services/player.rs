use colored::Colorize;

use crate::{
    entities::{capacity::Capacity, player::Player},
    utils::input,
};

pub fn create(capacities: Vec<Capacity>) -> Player {
    let name = input::readline(format!("Narrator: What is your name?").yellow().to_string());

    let player = Player {
        name,
        level: 1,
        health: 100,
        attack: 10,
        defense: 8,
        capacity: capacities,
    };

    return player;
}
