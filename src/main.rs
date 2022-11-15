use colored::Colorize;
use utils::{display, input};

use crate::services::game;

mod entities;
mod services;
mod utils;

fn main() {
    display::opening();
    let game = game::create();

    loop {
        println!();
        println!("{}", format!("Narrator: What you want to do ?").yellow());
        let input = input::readline(String::from(
            "1) Display your statistics\n2) Nothing\n3) Nothing\n4) Nothing",
        ));
        println!();
        match input.as_str() {
            "1" => {
                game.player.print_stats();
            }
            "2" => {}
            "3" => {}
            "4" => {}
            _ => print!("Nope!"),
        }
    }
}
