use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    copies: u32,
}

#[derive(Debug)]
struct ParseCardError;

impl fmt::Display for ParseCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse card")
    }
}

impl Error for ParseCardError {}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        parts.next().ok_or(ParseCardError)?;

        let mut number_parts = parts.next().ok_or(ParseCardError)?.split(" | ");

        let winning_numbers = number_parts
            .next()
            .ok_or(ParseCardError)?
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap_or_default())
            .collect();

        let numbers = number_parts
            .next()
            .ok_or(ParseCardError)?
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap_or_default())
            .collect();

        Ok(Self {
            winning_numbers,
            numbers,
            copies: 1,
        })
    }
}

impl Card {
    fn points(&self) -> u32 {
        self.numbers.iter().fold(0, |acc, n| {
            if self.winning_numbers.contains(n) {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        })
    }

    fn matching_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

pub fn sum_points(input: impl Read) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    for line in io::BufReader::new(input).lines() {
        let card = line?.parse::<Card>()?;

        sum += card.points();
    }

    Ok(sum)
}

pub fn total_cards(input: impl Read) -> Result<u32, Box<dyn Error>> {
    let mut cards = vec![];

    for line in io::BufReader::new(input).lines() {
        cards.push(line?.parse::<Card>()?);
    }

    for i in 0..cards.len() {
        for c in (i + 1)..(i + 1 + cards[i].matching_numbers() as usize) {
            if c >= cards.len() {
                break;
            }

            cards[c].copies += cards[i].copies;
        }
    }

    Ok(cards.iter().map(|card| card.copies).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn sum_total_points() -> Result<(), Box<dyn Error>> {
        assert_eq!(13, sum_points(INPUT.as_bytes())?);

        Ok(())
    }

    #[test]
    fn sum_total_cards() -> Result<(), Box<dyn Error>> {
        assert_eq!(30, total_cards(INPUT.as_bytes())?);

        Ok(())
    }
}
