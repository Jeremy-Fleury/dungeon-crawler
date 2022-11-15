use colored::Colorize;

use super::capacity::Capacity;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: u8,

    pub health: u8,
    pub attack: u8,
    pub defense: u8,

    pub capacity: Vec<Capacity>,
}

impl Player {
    pub fn print_stats(&self) {
        println!("{}", format!("Statistics").blue());
        println!("level:\t {}", self.level);
        println!("health:\t {}", self.health);
        println!("attack:\t {}", self.attack);
        println!("defense: {}", self.defense);
    }
}
