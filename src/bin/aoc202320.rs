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
    modules: HashMap<String, ModuleType>,
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
                        if matches!(m, ModuleType::Broadcaster(_)) {
                            *m = (*m)
                                .clone()
                                .dump_convert(ModuleType::FlipFlop(FlipFlop::default()))
                        };
                        m.add_receivers(&receivers)
                    })
                    .or_insert(ModuleType::FlipFlop(FlipFlop {
                        receivers: receivers.clone(),
                        ..Default::default()
                    }));
                k
            } else if let Some(k) = module.strip_prefix('&') {
                self.modules
                    .entry(k.into())
                    .and_modify(|m| {
                        if matches!(m, ModuleType::Broadcaster(_)) {
                            *m = (*m)
                                .clone()
                                .dump_convert(ModuleType::Conjunction(Conjunction::default()))
                        };
                        m.add_receivers(&receivers)
                    })
                    .or_insert(ModuleType::Conjunction(Conjunction {
                        receivers: receivers.clone(),
                        ..Default::default()
                    }));
                k
            } else {
                self.modules
                    .entry(module.into())
                    .and_modify(|m| m.add_receivers(&receivers))
                    .or_insert(ModuleType::Broadcaster(Broadcaster {
                        receivers: receivers.clone(),
                        ..Default::default()
                    }));
                module
            };

            for receiver in receivers {
                self.modules
                    .entry(receiver.clone())
                    .and_modify(|m| m.add_sender(sender))
                    .or_insert(ModuleType::Dump(Dump {
                        senders: vec![sender.into()],
                        ..Default::default()
                    }));
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
    modules: HashMap<String, ModuleType>,
}

impl AocDay {
    fn send_pulses(&self, pulses: u64) -> u64 {
        let initial_state = State {
            modules: self.modules.clone(),
        };
        let mut state = initial_state.clone();
        let mut count = self.send_pulse(&mut state);
        let mut loop_count = 1;
        while state != initial_state && loop_count <= pulses {
            loop_count += 1;
            let pulse_count = self.send_pulse(&mut state);
            count.0 += pulse_count.0;
            count.1 += pulse_count.1;
        }
        count = (
            count.0 * (pulses / loop_count),
            count.1 * (pulses / loop_count),
        );
        for _ in 0..(pulses % loop_count) {
            let pulse_count = self.send_pulse(&mut state);
            count.0 += pulse_count.0;
            count.1 += pulse_count.1;
        }
        count.0 * count.1
    }
    fn send_pulse(&self, state: &mut State) -> (u64, u64) {
        let mut highs = 0;
        let mut lows = 1; // Initial pulse
        let mut queue = VecDeque::new();
        for recv in state.modules["broadcaster"].receivers() {
            lows += 1; // Pulse send from broadcaster
            queue.push_back(("broadcaster".to_string(), recv.clone(), Pulse::Low));
        }
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

trait PulseProcessor {
    fn process_pulse(&mut self, sender: &str, pulse: Pulse) -> Vec<(String, Pulse)>;
}

impl PulseProcessor for FlipFlop {
    fn process_pulse(&mut self, _sender: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        if pulse == Pulse::High {
            return Vec::new();
        }
        let pulse = if self.is_on { Pulse::Low } else { Pulse::High };
        self.is_on = !self.is_on;
        self.receivers.iter().map(|r| (r.clone(), pulse)).collect()
    }
}

impl PulseProcessor for Conjunction {
    fn process_pulse(&mut self, sender: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        self.senders.entry(sender.to_string()).and_modify(|p| {
            *p = pulse;
        });
        let pulse = if self.senders.values().all(|v| v == &Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };
        self.receivers.iter().map(|r| (r.clone(), pulse)).collect()
    }
}

impl PulseProcessor for Broadcaster {
    fn process_pulse(&mut self, _sender: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        self.receivers.iter().map(|r| (r.clone(), pulse)).collect()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    #[default]
    Low,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct FlipFlop {
    senders: Vec<String>,
    receivers: Vec<String>,
    is_on: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Conjunction {
    senders: HashMap<String, Pulse>,
    receivers: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Broadcaster {
    senders: Vec<String>,
    receivers: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Dump {
    senders: Vec<String>,
    receivers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Dump(Dump),
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl ModuleType {
    fn process_pulse(&mut self, sender: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        match self {
            ModuleType::Dump(_) => Vec::new(),
            ModuleType::Broadcaster(m) => m.process_pulse(sender, pulse),
            ModuleType::FlipFlop(m) => m.process_pulse(sender, pulse),
            ModuleType::Conjunction(m) => m.process_pulse(sender, pulse),
        }
    }
    fn add_sender(&mut self, sender: &str) {
        match self {
            ModuleType::Dump(d) => d.senders.push(sender.into()),
            ModuleType::Broadcaster(b) => b.senders.push(sender.into()),
            ModuleType::FlipFlop(f) => f.senders.push(sender.into()),
            ModuleType::Conjunction(c) => {
                c.senders.insert(sender.into(), Pulse::Low);
            }
        };
    }

    fn add_receivers(&mut self, receivers: &[String]) {
        match self {
            ModuleType::Dump(m) => m.receivers.extend(receivers.iter().map(|r| r.to_string())),
            ModuleType::Broadcaster(b) => {
                b.receivers.extend(receivers.iter().map(|r| r.to_string()))
            }
            ModuleType::FlipFlop(f) => f.receivers.extend(receivers.iter().map(|r| r.to_string())),
            ModuleType::Conjunction(c) => {
                c.receivers.extend(receivers.iter().map(|r| r.to_string()))
            }
        };
    }

    fn receivers(&self) -> &[String] {
        match self {
            ModuleType::Dump(d) => &d.receivers,
            ModuleType::Broadcaster(b) => &b.receivers,
            ModuleType::FlipFlop(f) => &f.receivers,
            ModuleType::Conjunction(c) => &c.receivers,
        }
    }

    fn dump_convert(self, other: ModuleType) -> Self {
        if let Self::Dump(b) = self {
            match other {
                ModuleType::Dump(_) => ModuleType::Dump(b),
                ModuleType::Broadcaster(_) => Self::Broadcaster(Broadcaster {
                    senders: b.senders,
                    receivers: b.receivers,
                }),
                ModuleType::FlipFlop(_) => Self::FlipFlop(FlipFlop {
                    senders: b.senders,
                    receivers: b.receivers,
                    is_on: false,
                }),
                ModuleType::Conjunction(_) => Self::Conjunction(Conjunction {
                    senders: HashMap::from_iter(b.senders.iter().map(|r| (r.clone(), Pulse::Low))),
                    receivers: b.receivers,
                }),
            }
        } else {
            self
        }
    }
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
