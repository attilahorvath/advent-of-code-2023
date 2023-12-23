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

    fn add_row(&mut self, row: &mut Vec<Tile>) {
        if self.width == 0 {
            self.width = row.len();
        }

        self.tiles.append(row);
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn get_row(&self, y: usize) -> &[Tile] {
        &self.tiles[(y * self.width)..((y + 1) * self.width)]
    }

    fn get_col(&self, x: usize) -> Vec<Tile> {
        (0..self.height())
            .map(|y| self.tiles[y * self.width + x])
            .collect::<Vec<_>>()
    }

    fn reflected_cols(&self, max_differences: usize) -> usize {
        'outer: for x in 0..(self.width - 1) {
            let mut differences = 0;

            for i in 0..(x + 1).min(self.width - x - 1) {
                differences += self
                    .get_col(x - i)
                    .iter()
                    .zip(self.get_col(x + 1 + i).iter())
                    .filter(|(a, b)| a != b)
                    .count();

                if differences > max_differences {
                    continue 'outer;
                }
            }

            if differences == max_differences {
                return x + 1;
            }
        }

        0
    }

    fn reflected_rows(&self, max_differences: usize) -> usize {
        'outer: for y in 0..(self.height() - 1) {
            let mut differences = 0;

            for i in 0..(y + 1).min(self.height() - y - 1) {
                differences += self
                    .get_row(y - i)
                    .iter()
                    .zip(self.get_row(y + 1 + i).iter())
                    .filter(|(a, b)| a != b)
                    .count();

                if differences > max_differences {
                    continue 'outer;
                }
            }

            if differences == max_differences {
                return y + 1;
            }
        }

        0
    }

    fn summarize(&self, max_differences: usize) -> usize {
        self.reflected_cols(max_differences) + 100 * self.reflected_rows(max_differences)
    }
}

pub fn sum_patterns(input: impl Read, max_differences: usize) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    let mut pattern = Pattern::new();

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        if l.is_empty() {
            sum += pattern.summarize(max_differences);
            pattern = Pattern::new();
        } else {
            let mut row = l.chars().map(|c| c.into()).collect::<Vec<_>>();
            pattern.add_row(&mut row);
        }
    }

    sum += pattern.summarize(max_differences);

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
