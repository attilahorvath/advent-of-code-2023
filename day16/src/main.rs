use std::error::Error;
use std::fs::File;

use day16::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Total tiles being energized starting from the top left: {}",
        energized_tiles(File::open("input.txt")?, false)?
    );

    println!(
        "Maximum tiles being energized starting from the edges: {}",
        energized_tiles(File::open("input.txt")?, true)?
    );

    Ok(())
}
