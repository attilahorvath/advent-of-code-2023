use std::error::Error;
use std::fs::File;

use day03::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Sum of part numbers: {}",
        sum_part_numbers(File::open("input.txt")?)?
    );

    println!(
        "Sum of gear ratios: {}",
        sum_gear_ratios(File::open("input.txt")?)?
    );

    Ok(())
}
