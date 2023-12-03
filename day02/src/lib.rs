use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct GameSet {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct ParseGameError;

impl fmt::Display for ParseGameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse game")
    }
}

impl Error for ParseGameError {}

impl FromStr for GameSet {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for color_count in s.split(", ") {
            let mut parts = color_count.split(" ");
            let count: u32 = parts.next().ok_or(ParseGameError)?.parse().map_err(|_| ParseGameError)?;
            let color = parts.next().ok_or(ParseGameError)?;

            match color {
                "red" => red_count = count,
                "green" => green_count = count,
                "blue" => blue_count = count,
                _ => return Err(ParseGameError),
            }
        }

        Ok(GameSet { red_count, green_count, blue_count })
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");

        let mut id_parts = parts.next().ok_or(ParseGameError)?.split(" ");
        id_parts.next().ok_or(ParseGameError)?;
        let id = id_parts.next().ok_or(ParseGameError)?.parse().map_err(|_| ParseGameError)?;

        let set_parts = parts.next().ok_or(ParseGameError)?;
        let mut sets = vec![];

        for set in set_parts.split("; ") {
            sets.push(set.parse()?);
        }

        Ok(Game { id, sets })
    }
}

impl GameSet {
    fn is_possible(&self) -> bool {
        self.red_count <= MAX_RED && self.green_count <= MAX_GREEN && self.blue_count <= MAX_BLUE
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(GameSet::is_possible)
    }

    fn sum_power(&self) -> u32 {
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;

        for set in &self.sets {
            if set.red_count > max_red {
                max_red = set.red_count;
            }

            if set.green_count > max_green {
                max_green = set.green_count;
            }

            if set.blue_count > max_blue {
                max_blue = set.blue_count;
            }
        }

        max_red * max_green * max_blue
    }
}

pub fn possible_games<T: Read>(input: T) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    for line in io::BufReader::new(input).lines() {
        let game = line?.parse::<Game>()?;

        if game.is_possible() {
            sum += game.id;
        }
    }

    Ok(sum)
}

pub fn power_sets<T: Read>(input: T) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    for line in io::BufReader::new(input).lines() {
        let game = line?.parse::<Game>()?;

        sum += game.sum_power()
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_possible_games() -> Result<(), Box<dyn Error>> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, possible_games(input.as_bytes())?);

        Ok(())
    }

    #[test]
    fn sum_power_sets() -> Result<(), Box<dyn Error>> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, power_sets(input.as_bytes())?);

        Ok(())
    }
}
