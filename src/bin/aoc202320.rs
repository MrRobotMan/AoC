use std::collections::HashMap;

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day20.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    modules: HashMap<String, ModuleType>,
    instructions: Vec<(String, Vec<String>)>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 20)
    }

    fn parse(&mut self) {
        for line in aoc::read_lines(&self.input) {
            let (module, connections) = line.split_once(" -> ").unwrap();

            let sender = if let Some(k) = module.strip_prefix('%') {
                self.modules
                    .insert(k.into(), ModuleType::FlipFlop(FlipFlop { is_on: false }));
                k.into()
            } else if let Some(k) = module.strip_prefix('&') {
                self.modules
                    .insert(k.into(), ModuleType::Conjunction(Conjunction::default()));
                k.into()
            } else {
                self.modules.insert(module.into(), ModuleType::Broadcaster);
                module.into()
            };
            self.instructions.push((
                sender,
                connections
                    .split(", ")
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>(),
            ));
            for (sender, receivers) in &self.instructions {
                for receiver in receivers {
                    let module = self.modules.get_mut(receiver);
                    if let Some(ModuleType::Conjunction(c)) = module {
                        c.status.insert(sender.clone(), Pulse::Low);
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

trait PulseProcessor {
    fn process_pulse(&self, pulse: Pulse, sender: &str) -> Option<(Pulse, String)>;
}

#[derive(Debug, Default)]
enum Pulse {
    High,
    #[default]
    Low,
}

#[derive(Default, Debug)]
struct FlipFlop {
    is_on: bool,
}

#[derive(Default, Debug)]
struct Conjunction {
    status: HashMap<String, Pulse>,
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_version1() {
        let mut day = AocDay {
            input: "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
                .into(),
            ..Default::default()
        };
        day.parse();
        let expected = 32_000_000;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_version2() {
        let mut day = AocDay {
            input: "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
                .into(),
            ..Default::default()
        };
        day.parse();
        let expected = 11_687_500;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
