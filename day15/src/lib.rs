use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

enum Step {
    Add(String, usize),
    Remove(String),
}

struct Lens {
    label: String,
    focal_length: usize,
}

struct Boxes {
    boxes: HashMap<usize, Vec<Lens>>,
}

#[derive(Debug)]
struct ParseStepError;

impl fmt::Display for ParseStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse step")
    }
}

impl Error for ParseStepError {}

impl FromStr for Step {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("=") {
            let mut parts = s.split("=");

            let label = parts.next().ok_or(ParseStepError)?.to_string();
            let focal_length = parts
                .next()
                .ok_or(ParseStepError)?
                .parse()
                .map_err(|_| ParseStepError)?;

            return Ok(Step::Add(label, focal_length));
        } else if s.ends_with("-") {
            let label = s.strip_suffix("-").ok_or(ParseStepError)?.to_string();

            return Ok(Step::Remove(label));
        }

        Err(ParseStepError)
    }
}

impl Lens {
    fn new(label: &str, focal_length: usize) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }
}

impl Boxes {
    fn new() -> Self {
        Self {
            boxes: HashMap::new(),
        }
    }

    fn process(&mut self, step: &Step) {
        match step {
            Step::Add(label, focal_length) => {
                let entry = self.boxes.entry(hash(label)).or_default();

                if let Some(index) = entry.iter().position(|a| &a.label == label) {
                    entry[index].focal_length = *focal_length;
                } else {
                    entry.push(Lens::new(label, *focal_length));
                }
            }
            Step::Remove(label) => {
                let entry = self.boxes.entry(hash(label)).or_default();

                if let Some(index) = entry.iter().position(|a| &a.label == label) {
                    entry.remove(index);
                }
            }
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .map(|(&box_number, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(index, lens)| (box_number + 1) * (index + 1) * lens.focal_length)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn hash(string: &str) -> usize {
    let mut value = 0;

    for c in string.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }

    value
}

pub fn sum_hash_values(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        sum += l.split(",").map(|step| hash(step)).sum::<usize>();
    }

    Ok(sum)
}

pub fn focusing_power(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let mut boxes = Boxes::new();

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        for step in l.split(",") {
            boxes.process(&step.parse()?);
        }
    }

    Ok(boxes.focusing_power())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEQUENCE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_string() -> Result<(), Box<dyn Error>> {
        assert_eq!(52, sum_hash_values("HASH".as_bytes())?);

        Ok(())
    }

    #[test]
    fn hash_sequence() -> Result<(), Box<dyn Error>> {
        assert_eq!(1320, sum_hash_values(SEQUENCE.as_bytes())?);

        Ok(())
    }

    #[test]
    fn sequence_focusing_power() -> Result<(), Box<dyn Error>> {
        assert_eq!(145, focusing_power(SEQUENCE.as_bytes())?);

        Ok(())
    }
}
