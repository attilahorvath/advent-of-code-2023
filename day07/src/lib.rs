use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

#[derive(Debug)]
struct ParseHandError;

impl fmt::Display for ParseHandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse hand")
    }
}

impl Error for ParseHandError {}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => Card::Joker,
        }
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        let cards = parts
            .next()
            .ok_or(ParseHandError)?
            .chars()
            .map(|c| c.into())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParseHandError)?;

        let bid = parts
            .next()
            .ok_or(ParseHandError)?
            .parse()
            .map_err(|_| ParseHandError)?;

        Ok(Self { cards, bid })
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut tally = HashMap::new();
        let mut joker_count = 0;

        for card in self.cards {
            if card == Card::Joker {
                joker_count += 1;
            } else {
                *tally.entry(card).or_insert(0) += 1;
            }
        }

        let mut card_counts = tally.values().cloned().collect::<Vec<_>>();
        card_counts.sort();

        let max_count = card_counts.pop().unwrap_or_default();
        let max_2_count = card_counts.pop().unwrap_or_default();

        if max_count + joker_count == 5 {
            HandType::FiveOfAKind
        } else if max_count + joker_count == 4 {
            HandType::FourOfAKind
        } else if max_count + joker_count == 3 && max_2_count == 2 {
            HandType::FullHouse
        } else if max_count + joker_count == 3 {
            HandType::ThreeOfAKind
        } else if max_count == 2 && max_2_count + joker_count == 2 {
            HandType::TwoPair
        } else if max_count + joker_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type().cmp(&other.hand_type()).then_with(|| {
            for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let card_ordering = card.cmp(other_card);

                if card_ordering.is_ne() {
                    return card_ordering;
                }
            }

            Ordering::Equal
        })
    }
}

pub fn total_winnings(input: impl Read, jokers: bool) -> Result<u64, Box<dyn Error>> {
    let mut hands = vec![];

    for line in io::BufReader::new(input).lines() {
        let mut l = line?;

        if jokers {
            l = l.replace("J", "*");
        }

        hands.push(l.parse::<Hand>()?);
    }

    hands.sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1) * hand.bid)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn total_winnings_without_jokers() -> Result<(), Box<dyn Error>> {
        assert_eq!(6440, total_winnings(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn total_winnings_with_jokers() -> Result<(), Box<dyn Error>> {
        assert_eq!(5905, total_winnings(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
