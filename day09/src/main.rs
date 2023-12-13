use std::error::Error;
use std::fs::File;

use day09::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Sum of values: {}",
        sum_values(File::open("input.txt")?, false)?
    );

    println!(
        "Sum of values backwards: {}",
        sum_values(File::open("input.txt")?, true)?
    );

    Ok(())
}
