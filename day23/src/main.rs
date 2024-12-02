use std::error::Error;
use std::fs::File;

use day23::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Steps of longest hike: {}",
        longest_hike_steps(File::open("input.txt")?)?
    );

    Ok(())
}
