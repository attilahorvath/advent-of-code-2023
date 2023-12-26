use std::error::Error;
use std::fs::File;

use day18::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Cubic meters of lava held: {}",
        total_lava_held(File::open("input.txt")?, false)?
    );

    println!(
        "Cubic meters of lava held with swapped instructions: {}",
        total_lava_held(File::open("input.txt")?, true)?
    );

    Ok(())
}
