use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    RoundRock,
    CubeRock,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Platform {
    spaces: Vec<Space>,
    width: usize,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            'O' => Space::RoundRock,
            '#' => Space::CubeRock,
            _ => Space::Empty,
        }
    }
}

impl Platform {
    fn new() -> Self {
        Self {
            spaces: vec![],
            width: 0,
        }
    }

    fn add_row(&mut self, row: &mut Vec<Space>) {
        if self.width == 0 {
            self.width = row.len();
        }

        self.spaces.append(row);
    }

    fn height(&self) -> usize {
        self.spaces.len() / self.width
    }

    fn cols(&self, direction: Direction) -> usize {
        if direction == Direction::North || direction == Direction::South {
            self.width
        } else {
            self.height()
        }
    }

    fn rows(&self, direction: Direction) -> usize {
        if direction == Direction::North || direction == Direction::South {
            self.height()
        } else {
            self.width
        }
    }

    fn x_coord(&self, direction: Direction, col: usize, row: usize) -> usize {
        match direction {
            Direction::North => col,
            Direction::South => col,
            Direction::East => self.width - row - 1,
            Direction::West => row,
        }
    }

    fn y_coord(&self, direction: Direction, col: usize, row: usize) -> usize {
        match direction {
            Direction::North => row,
            Direction::South => self.height() - row - 1,
            Direction::East => col,
            Direction::West => col,
        }
    }

    fn get_space(&self, direction: Direction, col: usize, row: usize) -> Space {
        let x = self.x_coord(direction, col, row);
        let y = self.y_coord(direction, col, row);

        self.spaces[y * self.width + x]
    }

    fn set_space(&mut self, direction: Direction, col: usize, row: usize, space: Space) {
        let x = self.x_coord(direction, col, row);
        let y = self.y_coord(direction, col, row);

        self.spaces[y * self.width + x] = space;
    }

    pub fn tilt(&mut self, direction: Direction) {
        for col in 0..self.cols(direction) {
            let mut empty = None;

            for row in 0..self.rows(direction) {
                match self.get_space(direction, col, row) {
                    Space::Empty => {
                        if empty.is_none() {
                            empty = Some(row);
                        }
                    }
                    Space::RoundRock => {
                        if let Some(e) = empty {
                            self.set_space(direction, col, e, Space::RoundRock);
                            self.set_space(direction, col, row, Space::Empty);

                            empty = None;

                            for i in e..=row {
                                if self.get_space(direction, col, i) == Space::Empty {
                                    empty = Some(i);
                                    break;
                                }
                            }
                        }
                    }
                    Space::CubeRock => {
                        empty = None;
                    }
                }
            }
        }
    }

    pub fn load(&self, direction: Direction) -> usize {
        let mut sum = 0;

        for row in 0..self.rows(direction) {
            for col in 0..self.cols(direction) {
                if self.get_space(direction, col, row) == Space::RoundRock {
                    sum += self.rows(direction) - row;
                }
            }
        }

        sum
    }

    pub fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    pub fn load_after_cycles(&mut self, direction: Direction, cycles: usize) -> usize {
        let mut snapshots = vec![];
        let mut loads = vec![];

        let pattern_start;

        loop {
            self.cycle();
            loads.push(self.load(direction));

            if let Some(i) = snapshots.iter().position(|s| s == &self.spaces) {
                pattern_start = i;
                break;
            }

            snapshots.push(self.spaces.clone());
        }

        let remainder = cycles - (pattern_start + 1);
        let pattern_len = loads.len() - (pattern_start + 1);

        loads[(remainder % pattern_len) + pattern_start]
    }
}

pub fn build_platform(input: impl Read) -> Result<Platform, Box<dyn Error>> {
    let mut platform = Platform::new();

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        let mut row = l.chars().map(|c| c.into()).collect::<Vec<_>>();
        platform.add_row(&mut row);
    }

    Ok(platform)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn load_on_north_after_one_tilt() -> Result<(), Box<dyn Error>> {
        let mut platform = build_platform(INPUT.as_bytes())?;
        platform.tilt(Direction::North);

        assert_eq!(136, platform.load(Direction::North));

        Ok(())
    }

    #[test]
    fn load_on_north_after_many_cycles() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            64,
            build_platform(INPUT.as_bytes())?.load_after_cycles(Direction::North, 1_000_000_000)
        );

        Ok(())
    }
}
