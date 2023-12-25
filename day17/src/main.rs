use std::error::Error;
use std::fs::File;

use day17::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Least heat loss for normal crucible: {}",
        min_heat_loss(File::open("input.txt")?, 1, 3)?
    );

    println!(
        "Least heat loss for ultra crucible: {}",
        min_heat_loss(File::open("input.txt")?, 4, 10)?
    );

    Ok(())
}
