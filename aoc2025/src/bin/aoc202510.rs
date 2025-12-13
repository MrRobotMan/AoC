use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use puzlib::Combinations;

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
        .fold(0, |acc, machine| acc + machine.configure_lights())
}

fn part2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .fold(0, |acc, machine| acc + machine.configure_joltage())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
    jolts: Vec<usize>,
}

impl Machine {
    fn configure_lights(&self) -> usize {
        let mut state = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_front((0, vec![]));
        let mut best = vec![];
        while let Some((cur, list)) = queue.pop_front() {
            for button in &self.buttons {
                let mut next = list.clone();
                next.push(*button);
                let value = button ^ cur;
                if value == self.lights && (best.is_empty() || best.len() > next.len()) {
                    best = next.clone();
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

    fn configure_joltage(&self) -> usize {
        let joltage_parity_patterns = self.parity_patterns();
        find_best_joltage(
            self.jolts.clone(),
            &joltage_parity_patterns,
            &mut HashMap::new(),
        )
    }

    fn parity_patterns(&self) -> HashMap<usize, HashMap<Vec<usize>, usize>> {
        let mut joltage_parity_patterns =
            HashMap::from_iter((0..(1 << self.jolts.len())).map(|p| (p, HashMap::new())));
        for buttons_pressed in 0..self.buttons.len() + 1 {
            for button_combination in self.buttons.choose(buttons_pressed) {
                let pattern = (0..self.jolts.len())
                    .rev()
                    .map(|idx| {
                        button_combination.iter().fold(0, |acc, button| {
                            acc + (button & 1 << idx).count_ones() as usize
                        })
                    })
                    .collect::<Vec<_>>();
                let parity_pattern = pattern.iter().fold(0, |acc, v| (acc << 1) + (v % 2));
                joltage_parity_patterns
                    .entry(parity_pattern)
                    .and_modify(|e| {
                        e.entry(pattern).or_insert(buttons_pressed);
                    });
            }
        }

        joltage_parity_patterns
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
fn find_best_joltage(
    target: Vec<usize>,
    patterns: &HashMap<usize, HashMap<Vec<usize>, usize>>,
    cache: &mut HashMap<Vec<usize>, usize>,
) -> usize {
    // Credit to u/tentchmascot for this method.
    // https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    if let Some(result) = cache.get(&target) {
        return *result;
    }
    if target.iter().all(|v| v == &0) {
        return 0;
    }
    let mut best = usize::MAX;
    let pp = target.iter().fold(0, |acc, v| (acc << 1) + (v % 2));
    for (pattern, cost) in &patterns[&pp] {
        if pattern.iter().zip(target.iter()).all(|(p, t)| p <= t) {
            let next_target = pattern
                .iter()
                .zip(target.iter())
                .map(|(p, t)| (t - p) / 2)
                .collect::<Vec<_>>();
            let next_res = find_best_joltage(next_target, patterns, cache);
            if next_res < usize::MAX / 2 {
                best = best.min(cost + 2 * next_res);
            }
        }
    }
    best
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
        assert_eq!(2, actual.configure_lights());
        assert_eq!(10, actual.configure_joltage());
    }

    #[test]
    fn test_patterns() {
        let machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse::<Machine>()
            .unwrap();
        let expected = HashMap::from([
            (
                0,
                HashMap::from([
                    (vec![0, 0, 0, 0], 0),
                    (vec![0, 0, 2, 2], 3),
                    (vec![2, 2, 2, 2], 4),
                ]),
            ),
            (
                1,
                HashMap::from([
                    (vec![0, 0, 0, 1], 1),
                    (vec![0, 0, 2, 1], 2),
                    (vec![2, 2, 2, 1], 4),
                    (vec![2, 2, 2, 3], 5),
                ]),
            ),
            (
                2,
                HashMap::from([
                    (vec![0, 0, 1, 0], 1),
                    (vec![0, 0, 1, 2], 2),
                    (vec![2, 2, 1, 2], 4),
                    (vec![2, 2, 3, 2], 5),
                ]),
            ),
            (
                3,
                HashMap::from([
                    (vec![0, 0, 1, 1], 1),
                    (vec![2, 2, 1, 1], 3),
                    (vec![2, 2, 3, 3], 6),
                ]),
            ),
            (
                4,
                HashMap::from([
                    (vec![0, 1, 0, 2], 2),
                    (vec![0, 1, 2, 2], 3),
                    (vec![2, 1, 2, 0], 3),
                    (vec![2, 1, 2, 2], 4),
                ]),
            ),
            (
                5,
                HashMap::from([
                    (vec![0, 1, 0, 1], 1),
                    (vec![2, 1, 2, 1], 3),
                    (vec![0, 1, 2, 3], 4),
                ]),
            ),
            (
                6,
                HashMap::from([
                    (vec![0, 1, 1, 2], 2),
                    (vec![2, 1, 1, 0], 2),
                    (vec![2, 1, 3, 2], 5),
                ]),
            ),
            (
                7,
                HashMap::from([
                    (vec![0, 1, 1, 1], 2),
                    (vec![0, 1, 1, 3], 3),
                    (vec![2, 1, 1, 1], 3),
                    (vec![2, 1, 3, 1], 4),
                ]),
            ),
            (
                8,
                HashMap::from([
                    (vec![1, 0, 2, 0], 2),
                    (vec![1, 2, 0, 2], 3),
                    (vec![1, 0, 2, 2], 3),
                    (vec![1, 2, 2, 2], 4),
                ]),
            ),
            (
                9,
                HashMap::from([
                    (vec![1, 2, 0, 1], 2),
                    (vec![1, 0, 2, 1], 2),
                    (vec![1, 2, 2, 3], 5),
                ]),
            ),
            (
                10,
                HashMap::from([
                    (vec![1, 0, 1, 0], 1),
                    (vec![1, 2, 1, 2], 3),
                    (vec![1, 0, 3, 2], 4),
                ]),
            ),
            (
                11,
                HashMap::from([
                    (vec![1, 0, 1, 1], 2),
                    (vec![1, 2, 1, 1], 3),
                    (vec![1, 0, 3, 1], 3),
                    (vec![1, 2, 1, 3], 4),
                ]),
            ),
            (
                12,
                HashMap::from([(vec![1, 1, 0, 0], 1), (vec![1, 1, 2, 2], 3)]),
            ),
            (
                13,
                HashMap::from([
                    (vec![1, 1, 0, 1], 2),
                    (vec![1, 1, 2, 1], 3),
                    (vec![1, 1, 2, 3], 4),
                ]),
            ),
            (
                14,
                HashMap::from([
                    (vec![1, 1, 1, 0], 2),
                    (vec![1, 1, 1, 2], 3),
                    (vec![1, 1, 3, 2], 4),
                ]),
            ),
            (
                15,
                HashMap::from([(vec![1, 1, 1, 1], 2), (vec![1, 1, 3, 3], 5)]),
            ),
        ]);
        assert_eq!(machine.parity_patterns(), expected);
    }
}
