use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

struct Vertex {
    x: u64,
    y: u64,
    z: u64,
}

struct Brick {
    from: Vertex,
    to: Vertex,
    supports: Vec<usize>,
}

pub struct Snapshot {
    bricks: Vec<Brick>,
    supporteds: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct ParseBrickError;

impl fmt::Display for ParseBrickError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse brick")
    }
}

impl Error for ParseBrickError {}

impl FromStr for Vertex {
    type Err = ParseBrickError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        let x = parts
            .next()
            .unwrap_or_default()
            .parse()
            .map_err(|_| ParseBrickError)?;

        let y = parts
            .next()
            .unwrap_or_default()
            .parse()
            .map_err(|_| ParseBrickError)?;

        let z = parts
            .next()
            .unwrap_or_default()
            .parse()
            .map_err(|_| ParseBrickError)?;

        Ok(Self { x, y, z })
    }
}

impl FromStr for Brick {
    type Err = ParseBrickError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("~");

        let from = parts.next().unwrap_or_default().parse::<Vertex>()?;
        let to = parts.next().unwrap_or_default().parse::<Vertex>()?;

        let (from, to) = if to.z > from.z {
            (from, to)
        } else {
            (to, from)
        };

        Ok(Self {
            from,
            to,
            supports: vec![],
        })
    }
}

impl Snapshot {
    fn new() -> Self {
        Self {
            bricks: vec![],
            supporteds: vec![],
        }
    }

    fn add_brick(&mut self, brick: Brick) {
        self.bricks.push(brick);
    }

    fn find_supports(&mut self) {
        self.bricks.sort_by(|a, b| a.from.z.cmp(&b.from.z));
        self.supporteds = vec![vec![]; self.bricks.len()];

        let mut tops = HashMap::new();

        for (index, brick) in self.bricks.iter_mut().enumerate() {
            let x_range = if brick.from.x < brick.to.x {
                (brick.from.x, brick.to.x)
            } else {
                (brick.to.x, brick.from.x)
            };

            let y_range = if brick.from.y < brick.to.y {
                (brick.from.y, brick.to.y)
            } else {
                (brick.to.y, brick.from.y)
            };

            let height = brick.to.z - brick.from.z;

            let mut min_z = 1;

            for x in x_range.0..=x_range.1 {
                for y in y_range.0..=y_range.1 {
                    if let Some(&(top, support)) = tops.get(&(x, y)) {
                        if top + 1 > min_z {
                            brick.supports.clear();
                        }

                        if top + 1 >= min_z {
                            brick.supports.push(support);
                            min_z = top + 1;
                        }
                    }
                }
            }

            brick.from.z = min_z;
            brick.to.z = min_z + height;

            brick.supports.sort();
            brick.supports.dedup();

            for &support in &brick.supports {
                self.supporteds[support].push(index);
                self.supporteds[support].sort();
                self.supporteds[support].dedup();
            }

            for x in x_range.0..=x_range.1 {
                for y in y_range.0..=y_range.1 {
                    tops.insert((x, y), (min_z + height, index));
                }
            }
        }
    }

    pub fn safe_to_disintegrate(&self) -> usize {
        let mut not_safe = HashSet::new();

        for brick in &self.bricks {
            if brick.supports.len() == 1 {
                not_safe.insert(brick.supports[0]);
            }
        }

        self.bricks.len() - not_safe.len()
    }

    fn would_fall_from(&self, index: usize) -> usize {
        let mut would_fall = HashSet::new();
        let mut supported = VecDeque::new();

        for &i in &self.supporteds[index] {
            supported.push_back(i);
        }

        while let Some(s) = supported.pop_front() {
            if self.bricks[s]
                .supports
                .iter()
                .filter(|&&support| support != index && !would_fall.contains(&support))
                .count()
                == 0
            {
                would_fall.insert(s);

                for &i in &self.supporteds[s] {
                    supported.push_back(i);
                }
            }
        }

        would_fall.len()
    }

    pub fn would_fall(&self) -> usize {
        (0..self.bricks.len())
            .map(|index| self.would_fall_from(index))
            .sum()
    }
}

pub fn build_snapshot(input: impl Read) -> Result<Snapshot, Box<dyn Error>> {
    let mut snapshot = Snapshot::new();

    for line in io::BufReader::new(input).lines() {
        snapshot.add_brick(line?.parse()?);
    }

    snapshot.find_supports();

    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SNAPSHOT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn count_bricks_safe_to_disintegrate() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            5,
            build_snapshot(SNAPSHOT.as_bytes())?.safe_to_disintegrate()
        );

        Ok(())
    }

    #[test]
    fn count_bricks_that_would_fall() -> Result<(), Box<dyn Error>> {
        assert_eq!(7, build_snapshot(SNAPSHOT.as_bytes())?.would_fall());

        Ok(())
    }
}
