use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Path,
            '^' => Tile::Slope(Direction::Up),
            'v' => Tile::Slope(Direction::Down),
            '<' => Tile::Slope(Direction::Left),
            '>' => Tile::Slope(Direction::Right),
            _ => Tile::Forest,
        }
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        match *self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

impl Tile {
    fn is_reachable_from(&self, direction: Direction) -> bool {
        match *self {
            Tile::Path => true,
            Tile::Forest => false,
            Tile::Slope(slope_dir) => slope_dir != direction.opposite(),
        }
    }
}

impl Map {
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

    fn get_tile(&self, pos: (usize, usize)) -> Tile {
        self.tiles[pos.1 * self.width + pos.0]
    }

    fn longest_hike_from(&self, mut pos: (usize, usize), mut direction: Direction, mut visited: HashSet<(usize, usize)>, root: bool) -> usize {
        // println!("{:?}", pos);
        let mut steps = 1;

        if !visited.insert(pos) {
            return 0;
        }

        loop {


            pos = direction.step(pos);

            if !visited.insert(pos) {
                // println!("{:?} visited", pos);
                return 0;
                // break;
            } else {
                // println!("{:?} added", pos);
            }

            if pos.1 == self.height() - 1 {
                break;
            }

            steps += 1;

            let valid_directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right].iter().filter(|&&dir| {
                dir != direction.opposite() && self.get_tile(dir.step(pos)).is_reachable_from(dir)
            }).cloned().collect::<Vec<_>>();

            match valid_directions.len() {
                0 => break,
                1 => direction = valid_directions[0],
                _ => {
                    // let mut i = 0;
                    // let mut v = HashSet::new();
                    let res = valid_directions.iter().map(|&dir| {
                    // println!("check {:?} {steps}", dir.step(pos));
                    let s = self.longest_hike_from(dir.step(pos), dir, visited.clone(), false);
                    // println!("checked {:?} {} {steps}", dir.step(pos), s);
                    // if s > i {
                    //     i = s;
                    //     v = vis.clone();
                    // }
                    s + steps
                }).max().unwrap_or_default();
                // visited.extend(v.iter());
                // return (res, visited);
                steps = res;
                break;


                // direction = *valid_directions.iter().last().unwrap();
            }
            }
        }

        // if root {
        //     for y in 0..self.height() {
        //         for x in 0..self.width {
        //             let c = if visited.contains(&(x, y)) { "O" } else if self.get_tile((x, y)) == Tile::Forest { "#" } else { "." };
        //             print!("{c}");
        //         }

        //         println!();
        //     }
        // }

        steps
    }

    fn longest_hike(&self) -> usize {
        self.longest_hike_from((1, 0), Direction::Down, HashSet::new(), true)
    }
}

pub fn longest_hike_steps(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let mut map = Map::new();

    for line in io::BufReader::new(input).lines() {
        map.add_row(line?.chars().map(|c| c.into()).collect());
    }

    Ok(map.longest_hike())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn find_longest_hike() -> Result<(), Box<dyn Error>> {
        assert_eq!(94, longest_hike_steps(MAP.as_bytes())?);

        Ok(())
    }
}
