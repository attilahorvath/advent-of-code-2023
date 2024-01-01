use std::error::Error;
use std::fs::File;

use day22::*;

fn main() -> Result<(), Box<dyn Error>> {
    let snapshot = build_snapshot(File::open("input.txt")?)?;

    println!(
        "Bricks safe to disintegrate: {}",
        snapshot.safe_to_disintegrate()
    );

    println!("Bricks that would fall: {}", snapshot.would_fall());

    Ok(())
}
