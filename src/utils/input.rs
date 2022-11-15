use colored::*;
use std::io::{self, Write};

pub fn readline(label: String) -> String {
    let mut line = String::new();

    println!("{}", label);
    print!("{}", format!("> ").yellow());
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut line) {
        Ok(_) => line.trim().to_string(),
        Err(_) => panic!("Error while reading line"),
    }
}
