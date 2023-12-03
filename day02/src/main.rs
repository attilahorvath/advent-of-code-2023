use std::error::Error;
use std::fs::File;

use day02::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Possible games: {}", possible_games(File::open("input.txt")?)?);
    println!("Sum power sets: {}", power_sets(File::open("input.txt")?)?);

    Ok(())
}
