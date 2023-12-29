use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

struct Pattern {
    tiles: Vec<Tile>,
    width: usize,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Ash,
            _ => Tile::Rock,
        }
    }
}

impl Pattern {
    fn new() -> Self {
        Self {
            tiles: vec![],
            width: 0,
        }
    }

    fn add_row(&mut self, mut row: Vec<Tile>) {
        if self.width == 0 {
            self.width = row.len();
        }

        self.tiles.append(&mut row);
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn tiles_in_col<'a>(&'a self, x: usize) -> impl Iterator<Item = &Tile> + 'a {
        self.tiles.iter().skip(x).step_by(self.width)
    }

    fn tiles_in_row<'a>(&'a self, y: usize) -> impl Iterator<Item = &Tile> + 'a {
        self.tiles.iter().skip(y * self.width).take(self.width)
    }

    fn reflected_cols(&self, differences: usize) -> usize {
        'outer: for x in 0..(self.width - 1) {
            let mut diff = 0;

            for i in 0..(x + 1).min(self.width - x - 1) {
                diff += self
                    .tiles_in_col(x - i)
                    .zip(self.tiles_in_col(x + 1 + i))
                    .filter(|(a, b)| a != b)
                    .count();

                if diff > differences {
                    continue 'outer;
                }
            }

            if diff == differences {
                return x + 1;
            }
        }

        0
    }

    fn reflected_rows(&self, differences: usize) -> usize {
        'outer: for y in 0..(self.height() - 1) {
            let mut diff = 0;

            for i in 0..(y + 1).min(self.height() - y - 1) {
                diff += self
                    .tiles_in_row(y - i)
                    .zip(self.tiles_in_row(y + 1 + i))
                    .filter(|(a, b)| a != b)
                    .count();

                if diff > differences {
                    continue 'outer;
                }
            }

            if diff == differences {
                return y + 1;
            }
        }

        0
    }

    fn summarize(&self, differences: usize) -> usize {
        self.reflected_cols(differences) + 100 * self.reflected_rows(differences)
    }
}

pub fn sum_patterns(input: impl Read, differences: usize) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    let mut pattern = Pattern::new();

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        if l.is_empty() {
            sum += pattern.summarize(differences);
            pattern = Pattern::new();
        } else {
            pattern.add_row(l.chars().map(|c| c.into()).collect());
        }
    }

    sum += pattern.summarize(differences);

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn sum_patterns_without_differences() -> Result<(), Box<dyn Error>> {
        assert_eq!(405, sum_patterns(INPUT.as_bytes(), 0)?);

        Ok(())
    }

    #[test]
    fn sum_patterns_with_smudges() -> Result<(), Box<dyn Error>> {
        assert_eq!(400, sum_patterns(INPUT.as_bytes(), 1)?);

        Ok(())
    }
}
