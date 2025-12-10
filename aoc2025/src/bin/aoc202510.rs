use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

fn main() {
    println!("---- 2025: 10 ----");
    let input = "aoc2025/inputs/day10.txt";
    println!("Parsing");
    let machines = parse(input);
    println!("Part 1: {}", part1(&machines));
    println!("Part 2: {}", part2(&machines));
}

fn parse<S: AsRef<std::path::Path> + std::fmt::Display>(input: S) -> Vec<Machine> {
    puzlib::read_lines(input)
        .iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .fold(0, |acc, machine| acc + machine.button_presses())
}

fn part2(_machines: &[Machine]) -> String {
    "Unsolved".into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
    jolts: Vec<usize>,
}

impl Machine {
    fn button_presses(&self) -> usize {
        let mut state = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_front((0, vec![]));
        let mut best = vec![];
        while let Some((cur, list)) = queue.pop_front() {
            for button in &self.buttons {
                let mut next = list.clone();
                next.push(*button);
                let value = button ^ cur;
                if value == self.lights {
                    if best.is_empty() || best.len() > next.len() {
                        best = next.clone();
                    }
                }
                let s = state.entry(value).or_insert(next.clone());
                if s.len() > next.len() || *s == next {
                    *s = next.clone();
                    queue.push_back((value, next));
                }
            }
        }
        best.len()
    }
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((lights, rest)) = s[1..].split_once(']') else {
            return Err("Could not find lights section.");
        };
        let Some((buttons, jolts)) = rest[1..rest.len() - 1].split_once('{') else {
            return Err("Could not split batteries and jolts");
        };
        let l = lights.len() - 1;
        let lights = lights
            .chars()
            .fold(0, |acc, ch| acc << 1 | if ch == '#' { 1 } else { 0 });
        let buttons = buttons
            .split_whitespace()
            .map(|button_set| {
                button_set[1..button_set.len() - 1]
                    .split(',')
                    .fold(0_usize, |acc, v| {
                        acc | 1_usize << (l - v.parse::<usize>().unwrap())
                    })
            })
            .collect();
        let jolts = jolts.split(',').map(|v| v.parse().unwrap()).collect();
        Ok(Machine {
            lights,
            buttons,
            jolts,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = Machine {
            lights: 6,
            buttons: vec![1, 5, 2, 3, 10, 12],
            jolts: vec![3, 5, 4, 7],
        };
        let actual = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse::<Machine>()
            .unwrap();
        assert_eq!(expected, actual);
        assert_eq!(2, actual.button_presses())
    }
}
