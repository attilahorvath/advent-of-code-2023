use std::error::Error;
use std::fs::File;

use day14::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut platform = build_platform(File::open("input.txt")?)?;
    platform.tilt(Direction::North);

    println!(
        "Total load on North after one tilt: {}",
        platform.load(Direction::North)
    );

    println!(
        "Total load on North after 1,000,000,000 cycles: {}",
        build_platform(File::open("input.txt")?)?
            .load_after_cycles(Direction::North, 1_000_000_000)
    );

    Ok(())
}
