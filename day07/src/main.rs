use std::error::Error;
use std::fs::File;

use day07::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Total winnings without jokers: {}",
        total_winnings(File::open("input.txt")?, false)?
    );

    println!(
        "Total winnings with jokers: {}",
        total_winnings(File::open("input.txt")?, true)?
    );

    Ok(())
}
