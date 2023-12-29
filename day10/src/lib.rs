use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    vertices: Vec<(i32, i32)>,
    path: Vec<(i32, i32)>,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'S' => Tile::Start,
            _ => Tile::Ground,
        }
    }
}

impl Tile {
    fn is_vertex(&self) -> bool {
        *self == Tile::NorthEast
            || *self == Tile::NorthWest
            || *self == Tile::SouthWest
            || *self == Tile::SouthEast
    }

    fn change_direction(&self, direction: Direction) -> Option<Direction> {
        match direction {
            Direction::North => match *self {
                Tile::NorthSouth => Some(Direction::North),
                Tile::SouthWest => Some(Direction::West),
                Tile::SouthEast => Some(Direction::East),
                _ => None,
            },
            Direction::South => match *self {
                Tile::NorthSouth => Some(Direction::South),
                Tile::NorthEast => Some(Direction::East),
                Tile::NorthWest => Some(Direction::West),
                _ => None,
            },
            Direction::East => match *self {
                Tile::EastWest => Some(Direction::East),
                Tile::NorthWest => Some(Direction::North),
                Tile::SouthWest => Some(Direction::South),
                _ => None,
            },
            Direction::West => match *self {
                Tile::EastWest => Some(Direction::West),
                Tile::NorthEast => Some(Direction::North),
                Tile::SouthEast => Some(Direction::South),
                _ => None,
            },
        }
    }
}

impl Direction {
    fn step(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match *self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
}

impl Map {
    fn new() -> Self {
        Self {
            tiles: vec![],
            width: 0,
            vertices: vec![],
            path: vec![],
        }
    }

    fn add_row(&mut self, mut row: Vec<Tile>) {
        if self.width == 0 {
            self.width = row.len();
        }

        self.tiles.append(&mut row);
    }

    fn get_tile(&self, (x, y): (i32, i32)) -> Option<Tile> {
        if x < 0
            || y < 0
            || x >= self.width as i32
            || y >= self.tiles.len() as i32 / self.width as i32
        {
            return None;
        }

        Some(self.tiles[(y * self.width as i32 + x) as usize])
    }

    fn get_coords(&self, index: usize) -> (i32, i32) {
        (
            index as i32 % self.width as i32,
            index as i32 / self.width as i32,
        )
    }

    fn distance(&self, (x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
        (x2 - x1).abs() + (y2 - y1).abs()
    }

    fn valid_direction(&self, coords: (i32, i32)) -> Option<Direction> {
        for direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if let Some(tile) = self.get_tile(direction.step(coords)) {
                if tile.change_direction(direction).is_some() {
                    return Some(direction);
                }
            }
        }

        None
    }

    fn find_vertices(&mut self) -> Option<()> {
        if let Some(start_index) = self.tiles.iter().position(|&tile| tile == Tile::Start) {
            let start_coords = self.get_coords(start_index);

            let mut coords = start_coords;
            let mut direction = self.valid_direction(coords)?;

            self.path.push(coords);
            self.vertices.push(coords);

            loop {
                coords = direction.step(coords);

                if coords == start_coords {
                    return Some(());
                }

                let tile = self.get_tile(coords)?;

                self.path.push(coords);

                if tile.is_vertex() {
                    self.vertices.push(coords);
                }

                direction = tile.change_direction(direction)?;
            }
        }

        None
    }

    pub fn steps_to_furthest(&self) -> Option<u32> {
        if self.vertices.is_empty() {
            return None;
        }

        let steps = self
            .vertices
            .windows(2)
            .map(|coords| self.distance(coords[0], coords[1]))
            .sum::<i32>() as u32
            + self.distance(self.vertices[self.vertices.len() - 1], self.vertices[0]) as u32;

        Some(steps / 2)
    }

    pub fn enclosed_tiles(&self) -> Option<i32> {
        if self.vertices.is_empty() {
            return None;
        }

        let mut enclosed = 0;
        let mut edges = vec![];

        for y in 0..(self.tiles.len() / self.width) as i32 {
            for (index, vertex) in self
                .vertices
                .iter()
                .enumerate()
                .filter(|(_, vertex)| vertex.1 == y as i32)
            {
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

                let prev = self.vertices[prev_index];
                let next = self.vertices[next_index];

                if prev.1 > y {
                    edges.push((vertex.0, y, prev.1));
                } else if next.1 > y {
                    edges.push((vertex.0, y, next.1));
                }
            }

            edges.retain(|edge: &(i32, i32, i32)| edge.2 > y);
            edges.sort();

            let mut inside = false;

            for x in 0..self.width as i32 {
                let in_path = self.path.contains(&(x, y));

                if edges.iter().any(|edge| edge.0 == x) {
                    inside = !inside;
                }

                if inside && !in_path {
                    enclosed += 1;
                }
            }
        }

        Some(enclosed)
    }
}

pub fn build_map(input: impl Read) -> Result<Map, Box<dyn Error>> {
    let mut map = Map::new();

    for line in io::BufReader::new(input).lines() {
        map.add_row(line?.chars().map(|c| c.into()).collect());
    }

    map.find_vertices();

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_steps_square_loop() -> Result<(), Box<dyn Error>> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(
            4,
            build_map(input.as_bytes())?
                .steps_to_furthest()
                .unwrap_or_default()
        );

        Ok(())
    }

    #[test]
    fn num_steps_complex_loop() -> Result<(), Box<dyn Error>> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(
            8,
            build_map(input.as_bytes())?
                .steps_to_furthest()
                .unwrap_or_default()
        );

        Ok(())
    }

    #[test]
    fn enclosed_tiles_simple() -> Result<(), Box<dyn Error>> {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(
            4,
            build_map(input.as_bytes())?
                .enclosed_tiles()
                .unwrap_or_default()
        );

        Ok(())
    }

    #[test]
    fn enclosed_tiles_squeeze() -> Result<(), Box<dyn Error>> {
        let input = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

        assert_eq!(
            4,
            build_map(input.as_bytes())?
                .enclosed_tiles()
                .unwrap_or_default()
        );

        Ok(())
    }

    #[test]
    fn enclosed_tiles_larger() -> Result<(), Box<dyn Error>> {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(
            8,
            build_map(input.as_bytes())?
                .enclosed_tiles()
                .unwrap_or_default()
        );

        Ok(())
    }

    #[test]
    fn enclosed_tiles_junk_pipes() -> Result<(), Box<dyn Error>> {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(
            10,
            build_map(input.as_bytes())?
                .enclosed_tiles()
                .unwrap_or_default()
        );

        Ok(())
    }
}
