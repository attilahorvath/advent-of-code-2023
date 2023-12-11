use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SeedRange {
    start: u64,
    length: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MapRange {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

struct Map {
    map_ranges: Vec<MapRange>,
}

struct Almanac {
    seed_ranges: Vec<SeedRange>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct ParseAlmanacError;

impl fmt::Display for ParseAlmanacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse almanac")
    }
}

impl Error for ParseAlmanacError {}

impl FromStr for MapRange {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        let destination_start = parts
            .next()
            .ok_or(ParseAlmanacError)?
            .parse()
            .map_err(|_| ParseAlmanacError)?;

        let source_start = parts
            .next()
            .ok_or(ParseAlmanacError)?
            .parse()
            .map_err(|_| ParseAlmanacError)?;

        let length = parts
            .next()
            .ok_or(ParseAlmanacError)?
            .parse()
            .map_err(|_| ParseAlmanacError)?;

        Ok(Self {
            destination_start,
            source_start,
            length,
        })
    }
}

impl SeedRange {
    fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }
}

impl Map {
    fn new() -> Self {
        Self { map_ranges: vec![] }
    }

    fn map_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        let mut seed_ranges = vec![];
        let mut index = seed_range.start;

        let mut map_ranges = self
            .map_ranges
            .iter()
            .skip_while(|range| range.source_start + range.length < seed_range.start);

        let mut map_range = map_ranges.next();

        loop {
            let start;
            let mut length;

            if let Some(r) = map_range {
                if r.source_start > index {
                    start = index;
                    length = r.source_start - index;
                } else if r.source_start + r.length < seed_range.start + seed_range.length {
                    start = r.destination_start + (index - r.source_start);
                    length = r.source_start + r.length - index;

                    map_range = map_ranges.next();
                } else {
                    start = r.destination_start + (index - r.source_start);
                    length = seed_range.start + seed_range.length - index;
                }
            } else {
                start = index;
                length = seed_range.length - (index - seed_range.start);
            }

            if length > seed_range.start + seed_range.length - index {
                length = seed_range.start + seed_range.length - index;
            }

            index += length;
            seed_ranges.push(SeedRange::new(start, length));

            if index >= seed_range.start + seed_range.length || map_range.is_none() {
                break;
            }
        }

        seed_ranges
    }
}

impl Almanac {
    fn new() -> Self {
        Self {
            seed_ranges: vec![],
            maps: vec![],
        }
    }

    fn add_seeds(&mut self, s: &str, seed_ranges: bool) {
        let mut parts = s.split(": ");
        parts.next().unwrap_or_default();

        let mut seed_ranges = parts
            .next()
            .unwrap_or_default()
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap_or_default())
            .collect::<Vec<_>>()
            .chunks(if seed_ranges { 2 } else { 1 })
            .map(|chunk| SeedRange {
                start: chunk[0],
                length: if seed_ranges { chunk[1] } else { 1 },
            })
            .collect();

        self.seed_ranges.append(&mut seed_ranges);
    }

    fn add_map(&mut self) {
        self.maps.push(Map::new());
    }

    fn add_range(&mut self, range: MapRange) {
        if let Some(m) = self.maps.last_mut() {
            m.map_ranges.push(range);
            m.map_ranges.sort();
        }
    }

    fn min_range_start(&self) -> u64 {
        self.seed_ranges
            .iter()
            .map(|&seed_range| {
                let mut seed_ranges = vec![seed_range];
                for map in &self.maps {
                    let mut new_ranges = vec![];

                    for s in seed_ranges {
                        let mut r = map.map_range(&s);
                        new_ranges.append(&mut r);
                    }

                    seed_ranges = new_ranges;
                }
                seed_ranges
            })
            .flatten()
            .map(|r| r.start)
            .min()
            .unwrap_or_default()
    }
}

pub fn min_location(input: impl Read, seed_ranges: bool) -> Result<u64, Box<dyn Error>> {
    let mut almanac = Almanac::new();

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        if l.is_empty() {
            continue;
        }

        if l.contains("seeds:") {
            almanac.add_seeds(&l, seed_ranges);
        } else if l.contains("map:") {
            almanac.add_map();
        } else {
            almanac.add_range(l.parse()?);
        }
    }

    Ok(almanac.min_range_start())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn min_location_without_ranges() -> Result<(), Box<dyn Error>> {
        assert_eq!(35, min_location(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn min_location_with_ranges() -> Result<(), Box<dyn Error>> {
        assert_eq!(46, min_location(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
