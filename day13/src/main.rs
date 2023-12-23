use std::error::Error;
use std::fs::File;

use day13::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Sum of all patterns: {}",
        sum_patterns(File::open("input.txt")?, 0)?
    );

    println!(
        "Sum of all patterns with smudges: {}",
        sum_patterns(File::open("input.txt")?, 1)?
    );

    Ok(())
}
