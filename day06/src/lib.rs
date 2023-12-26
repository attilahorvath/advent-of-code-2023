use std::error::Error;
use std::io::{self, BufRead, Read};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn ways(&self) -> u64 {
        let b = self.time as f64;
        let c = self.distance as f64 + 0.5;
        let d = (b * b - 4.0 * c).sqrt();
        let min = (-b + d) / -2.0;
        let max = (-b - d) / -2.0;

        (max.floor() - min.ceil() + 1.0) as u64
    }
}

pub fn total_ways(input: impl Read, join: bool) -> Result<u64, Box<dyn Error>> {
    let mut times = vec![];
    let mut distances = vec![];

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        if l.starts_with("Time:") {
            let mut parts = l.split(":");
            parts.next();
            let times_parts = parts.next().unwrap_or_default().split_ascii_whitespace();

            if join {
                times = vec![times_parts.collect::<String>().parse().unwrap_or_default()];
            } else {
                times = times_parts
                    .map(|number| number.parse().unwrap_or_default())
                    .collect();
            }
        } else if l.starts_with("Distance:") {
            let mut parts = l.split(":");
            parts.next();
            let distances_parts = parts.next().unwrap_or_default().split_ascii_whitespace();

            if join {
                distances = vec![distances_parts
                    .collect::<String>()
                    .parse()
                    .unwrap_or_default()];
            } else {
                distances = distances_parts
                    .map(|number| number.parse().unwrap_or_default())
                    .collect();
            }
        }
    }

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race::new(time, distance))
        .collect::<Vec<_>>();

    Ok(races
        .iter()
        .map(|race| race.ways())
        .fold(1, |acc, ways| acc * ways))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn total_ways_without_join() -> Result<(), Box<dyn Error>> {
        assert_eq!(288, total_ways(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn total_ways_with_join() -> Result<(), Box<dyn Error>> {
        assert_eq!(71_503, total_ways(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
