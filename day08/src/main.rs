use std::error::Error;
use std::fs::File;

use day08::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Steps needed: {}",
        total_steps(File::open("input.txt")?, false)?
    );

    println!(
        "Steps needed with multiple paths: {}",
        total_steps(File::open("input.txt")?, true)?
    );

    Ok(())
}
