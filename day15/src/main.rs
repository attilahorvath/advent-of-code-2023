use std::error::Error;
use std::fs::File;

use day15::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Sum of all hashes: {}",
        sum_hash_values(File::open("input.txt")?)?
    );

    println!(
        "Total focusing power: {}",
        focusing_power(File::open("input.txt")?)?
    );

    Ok(())
}
