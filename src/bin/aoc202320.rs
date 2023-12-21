use std::collections::{HashMap, VecDeque};

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
    modules: HashMap<String, Module>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 20)
    }

    fn parse(&mut self) {
        for line in aoc::read_lines(&self.input) {
            let (module, connections) = line.split_once(" -> ").unwrap();
            let receivers = connections
                .split(", ")
                .map(|c| c.to_string())
                .collect::<Vec<_>>();
            let sender = if let Some(k) = module.strip_prefix('%') {
                self.modules
                    .entry(k.into())
                    .and_modify(|m| {
                        m.configuration = Configuration::FlipFlop;
                        m.receivers.extend(receivers.clone());
                    })
                    .or_insert(Module {
                        receivers: receivers.clone(),
                        configuration: Configuration::FlipFlop,
                        ..Default::default()
                    });
                k
            } else if let Some(k) = module.strip_prefix('&') {
                self.modules
                    .entry(k.into())
                    .and_modify(|m| {
                        m.configuration = Configuration::Conjunction;
                        m.receivers.extend(receivers.clone());
                    })
                    .or_insert(Module {
                        receivers: receivers.clone(),
                        configuration: Configuration::Conjunction,
                        ..Default::default()
                    });
                k
            } else {
                self.modules
                    .entry(module.into())
                    .and_modify(|m| {
                        m.configuration = Configuration::Broadcaster;
                        m.receivers.extend(receivers.clone());
                    })
                    .or_insert(Module {
                        receivers: receivers.clone(),
                        configuration: Configuration::Broadcaster,
                        ..Default::default()
                    });
                module
            };

            for receiver in receivers {
                self.modules
                    .entry(receiver.clone())
                    .and_modify(|m| {
                        m.senders.insert(sender.into(), Pulse::Low);
                    })
                    .or_insert(Module {
                        senders: HashMap::from([(sender.into(), Pulse::Low)]),
                        receivers: Vec::new(),
                        is_on: false,
                        configuration: Configuration::Dump,
                    });
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.send_pulses(1000))
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    modules: HashMap<String, Module>,
}

impl State {
    fn _dump(&self) {
        for (n, m) in self.modules.iter() {
            println!("{n}: is_on: {}, senders: {:?}", m.is_on, m.senders);
        }
    }
}

impl AocDay {
    fn send_pulses(&self, pulses: u64) -> u64 {
        let initial_state = State {
            modules: self.modules.clone(),
        };
        let mut state = initial_state.clone();
        let (mut low, mut high) = self.send_pulse(&mut state);
        let mut count = 1;
        while state != initial_state && count < pulses {
            count += 1;
            let pulse = self.send_pulse(&mut state);
            low += pulse.0;
            high += pulse.1;
        }
        low *= pulses / count;
        high *= pulses / count;
        for _ in 0..(pulses % count) {
            let pulse = self.send_pulse(&mut state);
            low += pulse.0;
            high += pulse.1;
        }
        low * high
    }

    fn send_pulse(&self, state: &mut State) -> (u64, u64) {
        let mut highs = 0;
        let mut lows = 1; // Initial pulse to broadcaster
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            for response in state
                .modules
                .get_mut(&receiver)
                .unwrap()
                .process_pulse(&sender, pulse)
            {
                match response.1 {
                    Pulse::High => highs += 1,
                    Pulse::Low => lows += 1,
                }
                queue.push_back((receiver.clone(), response.0, response.1));
            }
        }
        (lows, highs)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Module {
    senders: HashMap<String, Pulse>,
    receivers: Vec<String>,
    is_on: bool,
    configuration: Configuration,
}

impl Module {
    fn process_pulse(&mut self, sender: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        let sending = match self.configuration {
            Configuration::Broadcaster => pulse,
            Configuration::Conjunction => {
                self.senders.entry(sender.to_string()).and_modify(|p| {
                    *p = pulse;
                });
                if self.senders.values().all(|v| v == &Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            Configuration::FlipFlop => {
                if pulse == Pulse::High {
                    return Vec::new();
                } else {
                    let pulse = if self.is_on { Pulse::Low } else { Pulse::High };
                    self.is_on = !self.is_on;
                    pulse
                }
            }
            Configuration::Dump => return Vec::new(),
        };
        self.receivers
            .iter()
            .map(|r| (r.clone(), sending))
            .collect()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Configuration {
    Broadcaster,
    Conjunction,
    FlipFlop,
    #[default]
    Dump,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    #[default]
    Low,
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
