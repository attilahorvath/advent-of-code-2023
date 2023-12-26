use std::error::Error;
use std::fs::File;

use day19::*;

fn main() -> Result<(), Box<dyn Error>> {
    let processor = build_processor(File::open("input.txt")?)?;

    println!(
        "Sum of ratings of all accepted parts: {}",
        processor.sum_accepted()
    );

    println!(
        "Distinct combinations of ratings for accepted parts: {}",
        processor.sum_accepted_combinations()
    );

    Ok(())
}
