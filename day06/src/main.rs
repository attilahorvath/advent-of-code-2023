use std::error::Error;
use std::fs::File;

use day06::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Total number of ways without joining the numbers: {}",
        total_ways(File::open("input.txt")?, false)?
    );

    println!(
        "Total number of ways with joining the numbers: {}",
        total_ways(File::open("input.txt")?, true)?
    );

    Ok(())
}
