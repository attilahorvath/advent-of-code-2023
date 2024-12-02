use std::error::Error;
use std::fs::File;

use day12::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Sum of possible arrangement counts: {}",
        sum_counts(File::open("input.txt")?)?
    );

    Ok(())
}
