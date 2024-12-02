use std::error::Error;
use std::fs::File;

use day21::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Garden plots reachable in 64 steps: {}",
        count_reachable_garden_plots(File::open("input.txt")?, 64)?
    );

    Ok(())
}
