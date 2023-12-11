use std::error::Error;
use std::fs::File;

use day05::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Lowest location without seed ranges: {}",
        min_location(File::open("input.txt")?, false)?
    );

    println!(
        "Lowest location with seed ranges: {}",
        min_location(File::open("input.txt")?, true)?
    );

    Ok(())
}
