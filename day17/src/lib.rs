use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Step {
    loss: u32,
    pos: (usize, usize),
    direction: Direction,
}

struct Map {
    blocks: Vec<u32>,
    width: usize,
}

impl Direction {
    fn next_directions(&self) -> [Direction; 2] {
        if *self == Direction::Up || *self == Direction::Down {
            [Direction::Left, Direction::Right]
        } else {
            [Direction::Up, Direction::Down]
        }
    }

    fn advance(&self, pos: (usize, usize), by: usize, map: &Map) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if pos.0 > by - 1 {
                    Some((pos.0 - by, pos.1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if pos.0 + by < map.width {
                    Some((pos.0 + by, pos.1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if pos.1 > by - 1 {
                    Some((pos.0, pos.1 - by))
                } else {
                    None
                }
            }
            Direction::Right => {
                if pos.1 + by < map.height() {
                    Some((pos.0, pos.1 + by))
                } else {
                    None
                }
            }
        }
    }
}

impl Step {
    fn new(loss: u32, pos: (usize, usize), direction: Direction) -> Self {
        Self {
            loss,
            pos,
            direction,
        }
    }

    fn next_steps(&self, map: &Map, min_blocks: usize, max_blocks: usize) -> Vec<Step> {
        let mut steps = vec![];

        for direction in self.direction.next_directions() {
            let mut loss = 0;

            for i in 1..=max_blocks {
                if let Some(pos) = direction.advance(self.pos, i, map) {
                    loss += map.blocks[pos.1 * map.width + pos.0];

                    if i >= min_blocks {
                        steps.push(Step::new(self.loss + loss, pos, direction));
                    }
                }
            }
        }

        steps
    }
}

impl Map {
    fn new() -> Self {
        Self {
            blocks: vec![],
            width: 0,
        }
    }

    fn height(&self) -> usize {
        self.blocks.len() / self.width
    }

    fn add_row(&mut self, row: &mut Vec<u32>) {
        if self.width == 0 {
            self.width = row.len();
        }

        self.blocks.append(row);
    }

    fn find_min_loss(&self, min_blocks: usize, max_blocks: usize) -> u32 {
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut losses = HashMap::new();

        let mut min_loss = None;

        to_visit.push(Reverse(Step::new(0, (0, 0), Direction::Right)));
        to_visit.push(Reverse(Step::new(0, (0, 0), Direction::Down)));

        while let Some(Reverse(step)) = to_visit.pop() {
            if step.pos.0 == self.width - 1 && step.pos.1 == self.height() - 1 {
                if min_loss.is_none() || min_loss.unwrap_or_default() > step.loss {
                    min_loss = Some(step.loss);
                }
            }

            if !visited.insert((step.pos, step.direction)) {
                continue;
            }

            for next_step in step.next_steps(self, min_blocks, max_blocks) {
                let loss = losses.get(&(next_step.pos, next_step.direction));

                if loss.is_none() || loss.unwrap_or(&0) > &next_step.loss {
                    losses.insert((next_step.pos, next_step.direction), next_step.loss);
                    to_visit.push(Reverse(next_step));
                }
            }
        }

        min_loss.unwrap_or_default()
    }
}

pub fn min_heat_loss(
    input: impl Read,
    min_blocks: usize,
    max_blocks: usize,
) -> Result<u32, Box<dyn Error>> {
    let mut map = Map::new();

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        let mut row = l
            .chars()
            .map(|c| c.to_digit(10).unwrap_or_default())
            .collect();

        map.add_row(&mut row);
    }

    Ok(map.find_min_loss(min_blocks, max_blocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn minimize_heat_loss_normal_crucible() -> Result<(), Box<dyn Error>> {
        assert_eq!(102, min_heat_loss(MAP.as_bytes(), 1, 3)?);

        Ok(())
    }

    #[test]
    fn minimize_heat_loss_ultra_crucible() -> Result<(), Box<dyn Error>> {
        assert_eq!(94, min_heat_loss(MAP.as_bytes(), 4, 10)?);

        Ok(())
    }

    #[test]
    fn minimize_heat_loss_ultra_crucible_unfortunate_path() -> Result<(), Box<dyn Error>> {
        let map = "111111111111
999999999991
999999999991
999999999991
999999999991";

        assert_eq!(71, min_heat_loss(map.as_bytes(), 4, 10)?);

        Ok(())
    }
}
