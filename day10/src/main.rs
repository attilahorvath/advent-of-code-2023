use std::error::Error;
use std::fs::File;

use day10::*;

fn main() -> Result<(), Box<dyn Error>> {
    let map = build_map(File::open("input.txt")?)?;

    println!(
        "Number of steps to the farthest point: {}",
        map.steps_to_furthest().unwrap_or_default()
    );

    println!(
        "Enclosed tiles: {}",
        map.enclosed_tiles().unwrap_or_default()
    );

    Ok(())
}
