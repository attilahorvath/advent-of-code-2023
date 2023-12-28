use std::error::Error;
use std::fs::File;

use day20::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Total number of low pulses multiplied by the total number of high pulses after 1000 presses: {}",
        count_all_pulses(File::open("input.txt")?, 1000)?
    );

    println!(
        "Fewest number of presses to deliver a high pulse to ln, db, vq and tf at the same time: {}",
        count_cycle_length(File::open("input.txt")?, vec!["ln", "db", "vq", "tf"])?
    );

    Ok(())
}
