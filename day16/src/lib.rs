use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

struct Layout {
    tiles: Vec<Tile>,
    width: usize,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '/' => Tile::ForwardMirror,
            '\\' => Tile::BackwardMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            _ => Tile::Empty,
        }
    }
}

impl Beam {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn reflect_and_split(&mut self, tile: Tile) -> Option<Beam> {
        match tile {
            Tile::ForwardMirror => match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Down => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Down,
                Direction::Right => self.direction = Direction::Up,
            },
            Tile::BackwardMirror => match self.direction {
                Direction::Up => self.direction = Direction::Left,
                Direction::Down => self.direction = Direction::Right,
                Direction::Left => self.direction = Direction::Up,
                Direction::Right => self.direction = Direction::Down,
            },
            Tile::VerticalSplitter => {
                if self.direction == Direction::Left || self.direction == Direction::Right {
                    self.direction = Direction::Up;

                    return Some(Beam::new(self.x, self.y, Direction::Down));
                }
            }
            Tile::HorizontalSplitter => {
                if self.direction == Direction::Up || self.direction == Direction::Down {
                    self.direction = Direction::Left;

                    return Some(Beam::new(self.x, self.y, Direction::Right));
                }
            }
            _ => {}
        }

        None
    }

    fn advance(&mut self, layout: &Layout) -> bool {
        match self.direction {
            Direction::Up => {
                if self.y > 0 {
                    self.y -= 1;
                } else {
                    return false;
                }
            }
            Direction::Down => {
                if self.y < layout.height() - 1 {
                    self.y += 1;
                } else {
                    return false;
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    self.x -= 1;
                } else {
                    return false;
                }
            }
            Direction::Right => {
                if self.x < layout.width - 1 {
                    self.x += 1;
                } else {
                    return false;
                }
            }
        }

        true
    }
}

impl Layout {
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

    fn trace_beam(&self, beam: Beam, history: &mut HashMap<(usize, usize), [bool; 4]>) {
        let mut beam = beam;

        loop {
            let entry = history.entry((beam.x, beam.y)).or_default();

            if entry[beam.direction as usize] {
                return;
            }

            entry[beam.direction as usize] = true;

            if let Some(b) = beam.reflect_and_split(self.tiles[beam.y * self.width + beam.x]) {
                self.trace_beam(b, history);
            }

            if !beam.advance(self) {
                return;
            }
        }
    }

    fn find_energized(&self, beam: Beam) -> usize {
        let mut history = HashMap::new();

        self.trace_beam(beam, &mut history);

        history.len()
    }

    fn find_max_energized(&self) -> usize {
        let mut max_energized = 0;

        for x in 0..self.width {
            let max = self
                .find_energized(Beam::new(x, self.height() - 1, Direction::Up))
                .max(self.find_energized(Beam::new(x, 0, Direction::Down)));

            if max > max_energized {
                max_energized = max;
            }
        }

        for y in 0..self.height() {
            let max = self
                .find_energized(Beam::new(self.width - 1, 0, Direction::Left))
                .max(self.find_energized(Beam::new(0, y, Direction::Right)));

            if max > max_energized {
                max_energized = max;
            }
        }

        max_energized
    }
}

pub fn energized_tiles(input: impl Read, find_max: bool) -> Result<usize, Box<dyn Error>> {
    let mut layout = Layout::new();

    for line in io::BufReader::new(input).lines() {
        layout.add_row(line?.chars().map(|c| c.into()).collect());
    }

    if find_max {
        Ok(layout.find_max_energized())
    } else {
        Ok(layout.find_energized(Beam::new(0, 0, Direction::Right)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAYOUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn sum_energized_tiles_from_top_left() -> Result<(), Box<dyn Error>> {
        assert_eq!(46, energized_tiles(LAYOUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn sum_max_energized_tiles() -> Result<(), Box<dyn Error>> {
        assert_eq!(51, energized_tiles(LAYOUT.as_bytes(), true)?);

        Ok(())
    }
}
