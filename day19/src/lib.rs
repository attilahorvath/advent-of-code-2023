use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Rating {
    X,
    M,
    A,
    S,
}

enum Target {
    Accepted,
    Rejected,
    Workflow(String),
}

enum Rule {
    LessThan(Rating, u64, Target),
    MoreThan(Rating, u64, Target),
    SendTo(Target),
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

struct Part {
    ratings: HashMap<Rating, u64>,
}

pub struct Processor {
    workflows: HashMap<String, Workflow>,
    accepted: Vec<Part>,
}

#[derive(Debug)]
struct ParseWorkflowError;

impl fmt::Display for ParseWorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse workflow")
    }
}

impl Error for ParseWorkflowError {}

impl FromStr for Rating {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Rating::X),
            "m" => Ok(Rating::M),
            "a" => Ok(Rating::A),
            "s" => Ok(Rating::S),
            _ => Err(ParseWorkflowError),
        }
    }
}

impl FromStr for Target {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Target::Accepted),
            "R" => Ok(Target::Rejected),
            w => Ok(Target::Workflow(w.to_string())),
        }
    }
}

impl FromStr for Rule {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("<") || s.contains(">") {
            let less_than = s.contains("<");

            let mut parts = s.split(if less_than { "<" } else { ">" });
            let rating = parts.next().unwrap_or_default().parse()?;

            let mut parts = parts.next().unwrap_or_default().split(":");
            let value = parts
                .next()
                .unwrap_or_default()
                .parse()
                .map_err(|_| ParseWorkflowError)?;
            let target = parts.next().unwrap_or_default().parse()?;

            if less_than {
                Ok(Rule::LessThan(rating, value, target))
            } else {
                Ok(Rule::MoreThan(rating, value, target))
            }
        } else {
            Ok(Rule::SendTo(s.parse()?))
        }
    }
}

impl FromStr for Workflow {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim_end_matches("}").split("{");
        let name = parts.next().unwrap_or_default().to_string();
        let rules = parts
            .next()
            .unwrap_or_default()
            .split(",")
            .map(|p| p.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { name, rules })
    }
}

impl FromStr for Part {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Self::new();

        for r in s.trim_start_matches("{").trim_end_matches("}").split(",") {
            let mut parts = r.split("=");
            let rating = parts.next().unwrap_or_default().parse()?;
            let value = parts
                .next()
                .unwrap_or_default()
                .parse()
                .map_err(|_| ParseWorkflowError)?;

            part.ratings.insert(rating, value);
        }

        Ok(part)
    }
}

impl Part {
    fn new() -> Self {
        Self {
            ratings: HashMap::new(),
        }
    }

    fn sum_ratings(&self) -> u64 {
        self.ratings.values().sum()
    }
}

impl Processor {
    fn new() -> Self {
        Self {
            workflows: HashMap::new(),
            accepted: vec![],
        }
    }

    fn add_workflow(&mut self, workflow: Workflow) {
        self.workflows.insert(workflow.name.clone(), workflow);
    }

    fn process_part(&mut self, part: Part) {
        let mut workflow = &self.workflows["in"];

        loop {
            let mut new_target = None;

            for rule in &workflow.rules {
                match rule {
                    Rule::LessThan(rating, value, target) => {
                        if part.ratings.get(rating).unwrap_or(&0) < value {
                            new_target = Some(target);
                            break;
                        }
                    }
                    Rule::MoreThan(rating, value, target) => {
                        if part.ratings.get(rating).unwrap_or(&0) > value {
                            new_target = Some(target);
                            break;
                        }
                    }
                    Rule::SendTo(target) => {
                        new_target = Some(target);
                        break;
                    }
                };
            }

            if let Some(target) = new_target {
                match target {
                    Target::Accepted => {
                        self.accepted.push(part);
                        return;
                    }
                    Target::Rejected => {
                        return;
                    }
                    Target::Workflow(w) => {
                        workflow = &self.workflows[w];
                    }
                }
            }
        }
    }

    pub fn sum_accepted(&self) -> u64 {
        self.accepted.iter().map(|part| part.sum_ratings()).sum()
    }

    fn accepted_combinations(
        &self,
        workflow: &Workflow,
        mut curr_ratings: HashMap<Rating, (u64, u64)>,
        ranges: &mut Vec<HashMap<Rating, (u64, u64)>>,
    ) {
        for rule in &workflow.rules {
            match rule {
                Rule::LessThan(rating, value, target) => {
                    let mut new_ratings = curr_ratings.clone();

                    if let Some(r) = new_ratings.get_mut(rating) {
                        r.1 = *value - 1;
                    }

                    match target {
                        Target::Accepted => ranges.push(new_ratings),
                        Target::Rejected => {}
                        Target::Workflow(w) => {
                            self.accepted_combinations(&self.workflows[w], new_ratings, ranges)
                        }
                    }

                    if let Some(r) = curr_ratings.get_mut(rating) {
                        r.0 = *value;
                    }
                }
                Rule::MoreThan(rating, value, target) => {
                    let mut new_ratings = curr_ratings.clone();

                    if let Some(r) = new_ratings.get_mut(rating) {
                        r.0 = *value + 1;
                    }

                    match target {
                        Target::Accepted => ranges.push(new_ratings),
                        Target::Rejected => {}
                        Target::Workflow(w) => {
                            self.accepted_combinations(&self.workflows[w], new_ratings, ranges)
                        }
                    }

                    if let Some(r) = curr_ratings.get_mut(rating) {
                        r.1 = *value;
                    }
                }
                Rule::SendTo(target) => {
                    match target {
                        Target::Accepted => ranges.push(curr_ratings),
                        Target::Rejected => {}
                        Target::Workflow(w) => {
                            self.accepted_combinations(&self.workflows[w], curr_ratings, ranges)
                        }
                    }

                    return;
                }
            }
        }
    }

    pub fn sum_accepted_combinations(&self) -> u64 {
        let workflow = &self.workflows["in"];
        let curr_ratings = HashMap::from([
            (Rating::X, (1, 4000)),
            (Rating::M, (1, 4000)),
            (Rating::A, (1, 4000)),
            (Rating::S, (1, 4000)),
        ]);
        let mut ranges = vec![];

        self.accepted_combinations(workflow, curr_ratings, &mut ranges);

        ranges
            .iter()
            .map(|r| {
                r.values()
                    .map(|range| range.1 - range.0 + 1)
                    .fold(1, |acc, s| acc * s)
            })
            .sum()
    }
}

pub fn build_processor(input: impl Read) -> Result<Processor, Box<dyn Error>> {
    let mut processor = Processor::new();

    let mut reading_workflows = true;

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        if l.is_empty() {
            reading_workflows = false;
            continue;
        }

        if reading_workflows {
            processor.add_workflow(l.parse()?);
        } else {
            processor.process_part(l.parse()?);
        }
    }

    Ok(processor)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn count_all_accepted_part_ratings() -> Result<(), Box<dyn Error>> {
        let processor = build_processor(LIST.as_bytes())?;
        assert_eq!(19_114, processor.sum_accepted());

        Ok(())
    }

    #[test]
    fn count_all_accepted_combinations() -> Result<(), Box<dyn Error>> {
        let processor = build_processor(LIST.as_bytes())?;
        assert_eq!(167_409_079_868_000, processor.sum_accepted_combinations());

        Ok(())
    }
}
