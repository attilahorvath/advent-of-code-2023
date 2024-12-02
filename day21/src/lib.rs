use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    GardenPlot,
    Rock,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Start,
            '.' => Tile::GardenPlot,
            _ => Tile::Rock,
        }
    }
}

impl Tile {
    fn is_reachable(&self) -> bool {
        *self == Tile::Start || *self == Tile::GardenPlot
    }
}

impl Map {
    fn new() -> Self {
        Self {
            tiles: vec![],
            width: 0,
        }
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn add_row(&mut self, mut row: Vec<Tile>) {
        if self.width == 0 {
            self.width = row.len();
        }

        self.tiles.append(&mut row);
    }

    fn get_tile(&self, x: i64, y: i64) -> Tile {
        self.tiles[(y.rem_euclid(self.height() as i64) * self.width as i64
            + (x.rem_euclid(self.width as i64))) as usize]
    }

    fn starting_point(&self) -> Option<(i64, i64)> {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, &tile)| tile == Tile::Start)
            .map(|(index, _)| ((index % self.width) as i64, (index / self.width) as i64))
    }

    fn find_garden_plots2(&self, steps: usize) -> usize {
        let mut plots = vec![self.starting_point().unwrap_or_default()];

        for _ in 0..steps {
            let mut reachable = vec![];

            while let Some(pos) = plots.pop() {
                if self.get_tile(pos.0, pos.1 - 1).is_reachable() {
                    reachable.push((pos.0, pos.1 - 1));
                }

                if self.get_tile(pos.0, pos.1 + 1).is_reachable() {
                    reachable.push((pos.0, pos.1 + 1));
                }

                if self.get_tile(pos.0 - 1, pos.1).is_reachable() {
                    reachable.push((pos.0 - 1, pos.1));
                }

                if self.get_tile(pos.0 + 1, pos.1).is_reachable() {
                    reachable.push((pos.0 + 1, pos.1));
                }
            }

            reachable.sort();
            reachable.dedup();

            plots = reachable;
        }

        plots.len()
    }

    fn find_garden_plots(&self, _steps: usize) -> usize {
        let mut plots = HashMap::new();
        let mut next = VecDeque::from([(self.starting_point().unwrap_or_default(), 0)]);

        while let Some((pos, steps)) = next.pop_front() {
            if plots.contains_key(&pos) {
                continue;
            }

            if steps > 1000 {
                break;
            }

            plots.insert(pos, steps);

            // if pos.1 > self.height() as i64 * -3 && self.get_tile(pos.0, pos.1 - 1).is_reachable() {
            //     next.push_back(((pos.0, pos.1 - 1), steps + 1));
            // }

            // if pos.1 < self.height() as i64 * 3 - 1 && self.get_tile(pos.0, pos.1 + 1).is_reachable() {
            //     next.push_back(((pos.0, pos.1 + 1), steps + 1));
            // }

            // if pos.0 > self.width as i64 * -3 && self.get_tile(pos.0 - 1, pos.1).is_reachable() {
            //     next.push_back(((pos.0 - 1, pos.1), steps + 1));
            // }

            // if pos.0 < self.width as i64 * 3 - 1 && self.get_tile(pos.0 + 1, pos.1).is_reachable() {
            //     next.push_back(((pos.0 + 1, pos.1), steps + 1));
            // }

            if self.get_tile(pos.0, pos.1 - 1).is_reachable() {
                next.push_back(((pos.0, pos.1 - 1), steps + 1));
            }

            if self.get_tile(pos.0, pos.1 + 1).is_reachable() {
                next.push_back(((pos.0, pos.1 + 1), steps + 1));
            }

            if self.get_tile(pos.0 - 1, pos.1).is_reachable() {
                next.push_back(((pos.0 - 1, pos.1), steps + 1));
            }

            if self.get_tile(pos.0 + 1, pos.1).is_reachable() {
                next.push_back(((pos.0 + 1, pos.1), steps + 1));
            }
        }

        for y in (self.height() as i64 * -3)..(self.height() as i64 * 3) {
            if y % self.height() as i64 == 0 {
                println!("-");
            }
            for x in (self.width as i64 * -5)..(self.width as i64 * 1) {
                if x % self.width as i64 == 0 {
                    print!("|");
                }
                let c = if let Some(s) = plots.get(&(x as i64, y as i64)) {
                    format!("{s:2}")
                } else {
                    String::from(" .")
                };
                print!("{c} ");
            }

            println!();
        }

        0
    }
}

pub fn count_reachable_garden_plots(
    input: impl Read,
    steps: usize,
) -> Result<usize, Box<dyn Error>> {
    let mut map = Map::new();

    for line in io::BufReader::new(input).lines() {
        let row = line?.chars().map(|c| c.into()).collect();
        map.add_row(row);
    }

    Ok(map.find_garden_plots(steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn reachable_garden_plots_in_6_steps() -> Result<(), Box<dyn Error>> {
        assert_eq!(16, count_reachable_garden_plots(MAP.as_bytes(), 6)?);

        Ok(())
    }

    // #[test]
    // fn reachable_garden_plots_in_10_steps() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(50, count_reachable_garden_plots(MAP.as_bytes(), 10)?);

    //     Ok(())
    // }

    // #[test]
    // fn reachable_garden_plots_in_50_steps() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(1594, count_reachable_garden_plots(MAP.as_bytes(), 50)?);

    //     Ok(())
    // }

    // #[test]
    // fn reachable_garden_plots_in_100_steps() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(6536, count_reachable_garden_plots(MAP.as_bytes(), 100)?);

    //     Ok(())
    // }

    // #[test]
    // fn reachable_garden_plots_in_500_steps() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(6536, count_reachable_garden_plots(MAP.as_bytes(), 500)?);

    //     Ok(())
    // }
}
