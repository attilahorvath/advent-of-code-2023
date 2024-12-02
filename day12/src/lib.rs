use std::error::Error;
use std::fmt::{self, Debug};
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

struct Row {
    springs: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

#[derive(Debug)]
struct ParseRowError;

impl fmt::Display for ParseRowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse row")
    }
}

impl Error for ParseRowError {}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            _ => Condition::Unknown,
        }
    }
}

impl FromStr for Row {
    type Err = ParseRowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        let mut springs = parts
            .next()
            .ok_or(ParseRowError)?.to_string();

        springs += "?";

        let springs = springs.repeat(5).strip_suffix("?").unwrap()
            .chars()
            .map(|char| char.into())
            .collect::<Vec<_>>();

        let mut damaged_groups = parts
            .next()
            .ok_or(ParseRowError)?.to_string();

        damaged_groups += ",";

        let damaged_groups = damaged_groups.repeat(5).strip_suffix(",").unwrap()
            .split(",")
            .map(|group| group.parse().unwrap_or_default())
            .collect::<Vec<_>>();

        Ok(Self {
            springs,
            damaged_groups,
        })
    }
}

impl Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Condition::Operational => write!(f, "."),
            Condition::Damaged => write!(f, "D"),
            Condition::Unknown => write!(f, "?"),
        }
    }
}

impl Row {
    fn is_valid(&self, springs: &Vec<Condition>) -> bool {
        if springs
        .split(|&spring| spring == Condition::Operational)
        .filter(|group| !group.is_empty())
        .count() != self.damaged_groups.len() {
            return false;
        }
        springs
            .split(|&spring| spring == Condition::Operational)
            .filter(|group| !group.is_empty())
            .cycle()
            .zip(self.damaged_groups.iter())
            .all(|(spring_group, &damaged_group)| spring_group.len() == damaged_group)
    }

    fn count_arrangements_from(&self, curr_springs: Vec<Condition>, index: usize, damaged_group: usize, a: &mut Vec<Vec<Condition>>) -> u64 {
        for _ in 0..index {
            // print!(" ");
        }
        // println!("try: {:?} {} {}", curr_springs, index, damaged_group);
        if index >= self.springs.len() || damaged_group >= self.damaged_groups.len() {
            return 0;
        }

        let mut count = 0;

        'outer: for (i, &spring) in curr_springs[index..].iter().enumerate() {
            if index + i + self.damaged_groups[damaged_group] > curr_springs.len() {
                continue;
            }

            // println!("{index} {i} {damaged_group}");
            for j in 0..self.damaged_groups[damaged_group] {
                if curr_springs[index + i + j] == Condition::Operational {
                    // println!("already set {i}");
                    continue;
                }
            }

            if index + i + self.damaged_groups[damaged_group] + 1 < curr_springs.len()
                && curr_springs[index + i + self.damaged_groups[damaged_group]] == Condition::Damaged
            {
                // println!("a");
                continue;
            }

            let mut new_springs = curr_springs.clone();

            if i > 0 {
                for j in 1..i {
                    if new_springs[index + j - 1] == Condition::Unknown {
                        new_springs[index + j - 1] = Condition::Operational;
                    }
                }
            }

            // if index + i + self.damaged_groups[damaged_group] + 1 < self.springs.len()
            //     && new_springs[index + i + self.damaged_groups[damaged_group] + 1] == Condition::Damaged
            // {
                // println!("c {index} {i} {damaged_group}");
                // continue;
            // }

            for j in 0..self.damaged_groups[damaged_group] {
                if index + i + j >= self.springs.len() || new_springs[index + i + j] == Condition::Operational {
                    // println!("a {index} {i} {j}");
                    continue 'outer;
                }

                new_springs[index + i + j] = Condition::Damaged;
            }

            if damaged_group == self.damaged_groups.len() - 1 {
                for j in 0..new_springs.len() {
                    if new_springs[j] == Condition::Unknown {
                        new_springs[j] = Condition::Operational;
                    }
                }
            }

            if index + i > 0 {
                if new_springs[index + i - 1] == Condition::Damaged {
                    // println!("b");
                    continue 'outer;
                }

                new_springs[index + i - 1] = Condition::Operational;
            }

            if index + i + self.damaged_groups[damaged_group] > self.springs.len()
                // && new_springs[index + i + self.damaged_groups[damaged_group]] == Condition::Damaged
            {
                // continue;
                new_springs[index + i + self.damaged_groups[damaged_group]] = Condition::Operational;
            }


            if new_springs.iter().filter(|&&spring| spring == Condition::Unknown).count() > 0 {
                count += self.count_arrangements_from(new_springs, index + i + self.damaged_groups[damaged_group], damaged_group + 1, a);
            } else if self.is_valid(&new_springs) {
                a.push(new_springs);
                // println!("valid: {:?} {:?}", new_springs, self.damaged_groups);
                count += 1;
            } else {
                // println!("here");
                // if damaged_group == self.damaged_groups.len() - 1 {
                //     for j in 0..new_springs.len() {
                //         if new_springs[j] == Condition::Unknown {
                //             new_springs[j] = Condition::Operational;
                //         }
                //     }
                // }

                // if self.is_valid(&new_springs) {
                //     println!("valid: {:?} {:?}", new_springs, self.damaged_groups);
                //     count += 1;
                // }
                // println!("not valid");
            }
        }

        count
    }

    fn count_arrangements(&self) -> u64 {
        let mut a = vec![];
        println!("start: {:?} {:?}", self.springs, self.damaged_groups);
        let count = self.count_arrangements_from(self.springs.clone(), 0, 0, &mut a);
        a.sort();
        a.dedup();
        return a.len() as u64;
        if a.len() != count as usize {
            panic!("aaaaa");
        }
        println!("got {count}");
        count
    }
}

pub fn sum_counts(input: impl Read) -> Result<u64, Box<dyn Error>> {
    let mut sum = 0;

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        sum += l.parse::<Row>()?.count_arrangements();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    // const INPUT: &str = "?#?#?#?#?#?#?#? 1,3,1,6";
    // const INPUT: &str = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";

    #[test]
    fn sum_values_forward() -> Result<(), Box<dyn Error>> {
        assert_eq!(21, sum_counts(INPUT.as_bytes())?);

        Ok(())
    }
}
