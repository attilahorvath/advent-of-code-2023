use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Node {
    name: String,
    left: String,
    right: String,
}

struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
struct ParseMapError;

impl fmt::Display for ParseMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse map")
    }
}

impl Error for ParseMapError {}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            _ => Direction::Right,
        }
    }
}

impl FromStr for Node {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        let name = parts.next().ok_or(ParseMapError)?.to_string();

        let mut direction_parts = parts.next().ok_or(ParseMapError)?.split(", ");
        let left = direction_parts
            .next()
            .ok_or(ParseMapError)?
            .trim_start_matches("(")
            .to_string();
        let right = direction_parts
            .next()
            .ok_or(ParseMapError)?
            .trim_end_matches(")")
            .to_string();

        Ok(Self { name, left, right })
    }
}

impl Node {
    fn step(&self, direction: Direction) -> &String {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}

impl Map {
    fn new() -> Self {
        Self {
            directions: vec![],
            nodes: HashMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.directions.is_empty()
    }

    fn add_directions(&mut self, directions: &mut Vec<Direction>) {
        self.directions.append(directions);
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }

    fn steps_between(&self, start: String, end: String) -> u64 {
        let mut node = self.nodes.get(&start).unwrap();

        self.directions
            .iter()
            .cycle()
            .position(|&direction| {
                node = self.nodes.get(node.step(direction)).unwrap();

                node.name == end
            })
            .unwrap_or_default() as u64
            + 1
    }

    fn steps_to_multiple(&self) -> u64 {
        self.nodes
            .keys()
            .filter(|name| name.ends_with("A"))
            .map(|name| self.steps_to_any(name.to_string()))
            .fold(1, |acc, steps| lcm(acc, steps))
    }

    fn steps_to_any(&self, start: String) -> u64 {
        let mut node = self.nodes.get(&start).unwrap();

        self.directions
            .iter()
            .cycle()
            .position(|&direction| {
                node = self.nodes.get(node.step(direction)).unwrap();

                node.name.ends_with("Z")
            })
            .unwrap_or_default() as u64
            + 1
    }
}

pub fn total_steps(input: impl Read, multiple: bool) -> Result<u64, Box<dyn Error>> {
    let mut map = Map::new();

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        if l.is_empty() {
            continue;
        }

        if map.is_empty() {
            let mut directions = l.chars().map(|c| c.into()).collect();
            map.add_directions(&mut directions);
        } else {
            map.add_node(l.parse()?);
        }
    }

    if multiple {
        Ok(map.steps_to_multiple())
    } else {
        Ok(map.steps_between("AAA".to_string(), "ZZZ".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_steps_simple() -> Result<(), Box<dyn Error>> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, total_steps(input.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn total_steps_repeating() -> Result<(), Box<dyn Error>> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(6, total_steps(input.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn total_steps_multiple() -> Result<(), Box<dyn Error>> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(6, total_steps(input.as_bytes(), true)?);

        Ok(())
    }
}
