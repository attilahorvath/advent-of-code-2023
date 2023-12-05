use std::error::Error;
use std::fs::File;

use day01::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Calibration without strings: {}",
        calibrate(File::open("input.txt")?, false)?
    );

    println!(
        "Calibration with strings: {}",
        calibrate(File::open("input.txt")?, true)?
    );

    Ok(())
}
