use num::Integer;
use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, run_solution, Runner},
};

fn main() {
    let mut day = AocDay {
        input: "inputs/day08.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    nodes: HashMap<String, (String, String)>,
    instructions: Vec<char>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 8)
    }

    fn parse(&mut self) {
        let lines = read_lines(&self.input);
        self.instructions = lines[0].chars().collect();
        self.nodes = lines[1..]
            .iter()
            .map(|l| {
                let (node, dirs) = l.split_once(" = ").unwrap();
                let (left, right) = dirs[1..dirs.len() - 1].split_once(", ").unwrap();
                (node.into(), (left.into(), right.into()))
            })
            .collect::<HashMap<String, (String, String)>>();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.find_cycle("AAA".into()))
    }

    fn part2(&mut self) -> Vec<String> {
        output(
            self.nodes
                .keys()
                .filter_map(|k| {
                    if k.ends_with('A') {
                        Some(k.to_string())
                    } else {
                        None
                    }
                })
                .map(|c| self.find_cycle(c))
                .fold(1, |acc, c| acc.lcm(&c)),
        )
    }
}

impl AocDay {
    fn find_cycle(&self, mut current: String) -> u64 {
        let mut cycle = self.instructions.iter().cycle();
        let mut steps = 0;
        loop {
            match cycle.next().unwrap() {
                'L' => current = self.nodes[&current].0.clone(),
                'R' => current = self.nodes[&current].1.clone(),
                _ => panic!("Unknown instruction"),
            };
            steps += 1;
            if current.ends_with('Z') {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 2;
        let mut day = AocDay {
            nodes: [
                "AAA = (BBB, CCC)",
                "BBB = (DDD, EEE)",
                "CCC = (ZZZ, GGG)",
                "DDD = (DDD, DDD)",
                "EEE = (EEE, EEE)",
                "GGG = (GGG, GGG)",
                "ZZZ = (ZZZ, ZZZ)",
            ]
            .iter()
            .map(|l| {
                let (node, dirs) = l.split_once(" = ").unwrap();
                let (left, right) = dirs[1..dirs.len() - 1].split_once(", ").unwrap();
                println!("{node} -> ({left}, {right})");
                (node.into(), (left.into(), right.into()))
            })
            .collect::<HashMap<String, (String, String)>>(),
            instructions: "RL".chars().collect(),
        };
        let actual = day.part1()[0].parse().unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part1_alt_input() {
        let expected = 6;
        let mut day = AocDay {
            nodes: ["AAA = (BBB, BBB)", "BBB = (AAA, ZZZ)", "ZZZ = (ZZZ, ZZZ)"]
                .iter()
                .map(|l| {
                    let (node, dirs) = l.split_once(" = ").unwrap();
                    let (left, right) = dirs[1..dirs.len() - 1].split_once(", ").unwrap();
                    println!("{node} -> ({left}, {right})");
                    (node.into(), (left.into(), right.into()))
                })
                .collect::<HashMap<String, (String, String)>>(),
            instructions: "LLR".chars().collect(),
        };
        let actual = day.part1()[0].parse().unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part2() {
        let expected = 6;
        let mut day = AocDay {
            nodes: [
                "11A = (11B, XXX)",
                "11B = (XXX, 11Z)",
                "11Z = (11B, XXX)",
                "22A = (22B, XXX)",
                "22B = (22C, 22C)",
                "22C = (22Z, 22Z)",
                "22Z = (22B, 22B)",
                "XXX = (XXX, XXX)",
            ]
            .iter()
            .map(|l| {
                let (node, dirs) = l.split_once(" = ").unwrap();
                let (left, right) = dirs[1..dirs.len() - 1].split_once(", ").unwrap();
                println!("{node} -> ({left}, {right})");
                (node.into(), (left.into(), right.into()))
            })
            .collect::<HashMap<String, (String, String)>>(),
            instructions: "LR".chars().collect(),
        };
        let actual = day.part2()[0].parse().unwrap();
        assert_eq!(expected, actual)
    }
}
