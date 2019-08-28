mod rankings;
mod solver;

use solver::*;
use std::process::exit;

fn main() {
    let max_attempts = 1000;
    let result = find_ranking(max_attempts);
    match result {
        Ok(ranking) => {
            println!("A ranking of all the developers was found: {}", ranking);
        }
        Err(message) => {
            println!("Error: {}", message);
            exit(1);
        }
    }
}

