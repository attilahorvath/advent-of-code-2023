use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

enum ModuleType {
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<String, Pulse> },
    Broadcast,
}

struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
    next_pulse: Option<Pulse>,
}

struct Network {
    modules: HashMap<String, Module>,
}

#[derive(Debug)]
struct ParseModuleError;

impl fmt::Display for ParseModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse module")
    }
}

impl Error for ParseModuleError {}

impl FromStr for Module {
    type Err = ParseModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let module_type = match s.chars().next().unwrap_or_default() {
            '%' => ModuleType::FlipFlop { state: false },
            '&' => ModuleType::Conjunction {
                inputs: HashMap::new(),
            },
            _ => ModuleType::Broadcast,
        };

        let mut parts = s.split(" -> ");

        let name = parts
            .next()
            .unwrap_or_default()
            .trim_start_matches("%")
            .trim_start_matches("&")
            .to_string();

        let outputs = parts
            .next()
            .unwrap_or_default()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        Ok(Self {
            name,
            module_type,
            outputs,
            next_pulse: None,
        })
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}

impl Module {
    fn receive(&mut self, from: &str, pulse: Pulse) {
        match &mut self.module_type {
            ModuleType::FlipFlop { state } => {
                if pulse == Pulse::Low {
                    *state = !*state;

                    self.next_pulse = Some(if *state { Pulse::High } else { Pulse::Low });
                } else {
                    self.next_pulse = None;
                }
            }
            ModuleType::Conjunction { inputs } => {
                if let Some(p) = inputs.get_mut(from) {
                    *p = pulse;
                }

                self.next_pulse = Some(if inputs.values().all(|&v| v == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                });
            }
            ModuleType::Broadcast => {
                self.next_pulse = Some(pulse);
            }
        }
    }
}

impl Network {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    fn add_module(&mut self, mut module: Module) {
        if let ModuleType::Conjunction { inputs } = &mut module.module_type {
            for input in self
                .modules
                .values()
                .filter(|m| m.outputs.contains(&module.name))
            {
                inputs.insert(input.name.clone(), Pulse::Low);
            }
        }

        for output in &module.outputs {
            if let Some(o) = self.modules.get_mut(output) {
                if let ModuleType::Conjunction { inputs } = &mut o.module_type {
                    inputs.insert(module.name.clone(), Pulse::Low);
                }
            }
        }

        self.modules.insert(module.name.clone(), module);
    }

    fn reset(&mut self) {
        for module in self.modules.values_mut() {
            match &mut module.module_type {
                ModuleType::FlipFlop { state } => {
                    *state = false;
                }
                ModuleType::Conjunction { inputs } => {
                    for input in inputs.values_mut() {
                        *input = Pulse::Low;
                    }
                }
                _ => {}
            }
        }
    }

    fn broadcast(&mut self, pulse_counts: &mut HashMap<Pulse, u64>, probe: Option<&str>) -> bool {
        let mut pulses = VecDeque::new();

        pulses.push_back((
            String::from("button"),
            String::from("broadcaster"),
            Pulse::Low,
        ));

        while let Some((from, to, pulse)) = pulses.pop_front() {
            *pulse_counts.entry(pulse).or_insert(0) += 1;

            if let Some(module) = self.modules.get_mut(&to) {
                module.receive(&from, pulse);

                if let Some(next_pulse) = module.next_pulse {
                    if let Some(p) = probe {
                        if next_pulse == Pulse::High && module.name == p {
                            return true;
                        }
                    }

                    for output in &module.outputs {
                        pulses.push_back((module.name.clone(), output.clone(), next_pulse));
                    }
                }
            }
        }

        false
    }

    fn sum_pulses(&mut self, presses: u64) -> u64 {
        let mut pulse_counts = HashMap::new();

        self.reset();

        for _ in 0..presses {
            self.broadcast(&mut pulse_counts, None);
        }

        pulse_counts.values().fold(1, |acc, c| acc * c)
    }

    fn cycle_length(&mut self, probes: Vec<&str>) -> u64 {
        let mut cycles = vec![];
        let mut pulse_counts = HashMap::new();

        for probe in probes {
            let mut presses = 0;

            self.reset();

            loop {
                presses += 1;

                if self.broadcast(&mut pulse_counts, Some(probe)) {
                    break;
                }
            }

            cycles.push(presses);
        }

        cycles.iter().fold(1, |acc, &c| lcm(acc, c))
    }
}

pub fn count_all_pulses(input: impl Read, presses: u64) -> Result<u64, Box<dyn Error>> {
    let mut network = Network::new();

    for line in io::BufReader::new(input).lines() {
        network.add_module(line?.parse()?);
    }

    Ok(network.sum_pulses(presses))
}

pub fn count_cycle_length(input: impl Read, probes: Vec<&str>) -> Result<u64, Box<dyn Error>> {
    let mut network = Network::new();

    for line in io::BufReader::new(input).lines() {
        network.add_module(line?.parse()?);
    }

    Ok(network.cycle_length(probes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_all_pulses_simple() -> Result<(), Box<dyn Error>> {
        let configuration = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        assert_eq!(
            32_000_000,
            count_all_pulses(configuration.as_bytes(), 1000)?
        );

        Ok(())
    }

    #[test]
    fn count_all_pulses_flip_flops() -> Result<(), Box<dyn Error>> {
        let configuration = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(
            11_687_500,
            count_all_pulses(configuration.as_bytes(), 1000)?
        );

        Ok(())
    }
}
