use std::error::Error;
use std::fs::File;

use day11::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Sum of lengths with base rate: {}",
        sum_lengths(File::open("input.txt")?, 2)?
    );

    println!(
        "Sum of lengths with rate of 1,000,000: {}",
        sum_lengths(File::open("input.txt")?, 1_000_000)?
    );

    Ok(())
}
