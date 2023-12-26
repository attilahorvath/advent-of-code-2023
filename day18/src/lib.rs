use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    length: i64,
}

struct Digger {
    vertices: Vec<(i64, i64)>,
    pos: (i64, i64),
}

#[derive(Debug)]
struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse instruction")
    }
}

impl Error for ParseInstructionError {}

impl FromStr for Direction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap_or_default() {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(ParseInstructionError),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let direction = parts.next().ok_or(ParseInstructionError)?.parse()?;
        let length = parts
            .next()
            .ok_or(ParseInstructionError)?
            .parse()
            .map_err(|_| ParseInstructionError)?;

        Ok(Instruction { direction, length })
    }
}

impl Direction {
    fn swap(s: &str) -> String {
        match s.chars().next().unwrap_or_default() {
            '0' => String::from("R"),
            '1' => String::from("D"),
            '2' => String::from("L"),
            '3' => String::from("U"),
            _ => String::new(),
        }
    }
}

impl Instruction {
    fn swap(s: &str) -> String {
        let color = s
            .split_ascii_whitespace()
            .last()
            .unwrap_or_default()
            .trim_start_matches("(#")
            .trim_end_matches(")");

        let length = i64::from_str_radix(&color[..5], 16).unwrap_or_default();
        let direction = Direction::swap(&color[5..]);

        format!("{direction} {length}")
    }

    fn advance(&self, pos: (i64, i64)) -> (i64, i64) {
        match self.direction {
            Direction::Up => (pos.0, pos.1 - self.length),
            Direction::Down => (pos.0, pos.1 + self.length),
            Direction::Left => (pos.0 - self.length, pos.1),
            Direction::Right => (pos.0 + self.length, pos.1),
        }
    }
}

impl Digger {
    fn new() -> Self {
        Self {
            vertices: vec![(0, 0)],
            pos: (0, 0),
        }
    }

    fn process(&mut self, instruction: Instruction) {
        self.pos = instruction.advance(self.pos);
        self.vertices.push(self.pos);
    }

    fn lava_held(&self) -> i64 {
        let mut sum = 0;
        let mut edges = vec![];

        let mut ys = self
            .vertices
            .iter()
            .flat_map(|v| [v.1, v.1 + 1])
            .collect::<Vec<_>>();

        ys.sort();
        ys.dedup();

        let mut filled = 0;
        let mut prev_y = None;

        for y in ys {
            if let Some(py) = prev_y {
                sum += filled * (y - py);
            }

            prev_y = Some(y);
            filled = 0;

            for (index, vertex) in self.vertices.iter().enumerate().filter(|(_, v)| v.1 == y) {
                let prev_index = if index == 0 {
                    self.vertices.len() - 1
                } else {
                    index - 1
                };

                let next_index = if index == self.vertices.len() - 1 {
                    0
                } else {
                    index + 1
                };

                let other = if self.vertices[prev_index].0 == vertex.0 {
                    self.vertices[prev_index]
                } else {
                    self.vertices[next_index]
                };

                if vertex.1 < other.1 {
                    edges.push((vertex.0, vertex.1, other.1));
                }
            }

            edges.retain(|e| e.2 >= y);
            edges.sort();

            let mut inside = false;
            let mut inline = false;
            let mut top_edge = false;

            let mut start_x = None;

            for edge in &edges {
                if edge.1 == y || edge.2 == y {
                    if !inline {
                        inline = true;
                        top_edge = edge.1 == y;
                    } else {
                        inline = false;

                        if top_edge != (edge.1 == y) {
                            inside = !inside;
                        }
                    }
                } else {
                    inside = !inside;
                }

                if inside || inline {
                    if start_x.is_none() {
                        start_x = Some(edge.0);
                    }
                } else {
                    filled += edge.0 - start_x.unwrap_or_default() + 1;
                    start_x = None;
                }
            }
        }

        sum
    }
}

pub fn total_lava_held(input: impl Read, swapped: bool) -> Result<i64, Box<dyn Error>> {
    let mut digger = Digger::new();

    for line in io::BufReader::new(input).lines() {
        let mut l = line?;

        if swapped {
            l = Instruction::swap(&l);
        }

        let instruction = l.parse()?;

        digger.process(instruction);
    }

    Ok(digger.lava_held())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAN: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn count_total_lava_held() -> Result<(), Box<dyn Error>> {
        assert_eq!(62, total_lava_held(PLAN.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn count_total_lava_held_swapped() -> Result<(), Box<dyn Error>> {
        assert_eq!(952408144115, total_lava_held(PLAN.as_bytes(), true)?);

        Ok(())
    }
}
