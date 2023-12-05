use std::error::Error;
use std::fs::File;

use day04::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Sum points: {}", sum_points(File::open("input.txt")?)?);
    println!("Total cards: {}", total_cards(File::open("input.txt")?)?);

    Ok(())
}
