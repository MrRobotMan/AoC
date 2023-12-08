use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, run_solution, Runner},
};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    nodes: HashMap<String, (String, String)>,
    instructions: Vec<char>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 8)
    }

    fn parse(&mut self) {
        let lines = read_lines("inputs/2023/day08.txt");
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
        let mut cycle = self.instructions.iter().cycle();
        let mut current = "AAA".into();
        let mut steps = 0;
        loop {
            match cycle.next().unwrap() {
                'L' => current = self.nodes[&current].0.clone(),
                'R' => current = self.nodes[&current].1.clone(),
                _ => panic!("Unknown instruction"),
            };
            steps += 1;
            if current == "ZZZ" {
                return output(steps);
            }
        }
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CYCLE: &str = "RL";
    const NODES: [&str; 7] = [
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    #[test]
    fn test_part1() {
        let expected = 2;
        let mut day = AocDay {
            nodes: NODES
                .iter()
                .map(|l| {
                    let (node, dirs) = l.split_once(" = ").unwrap();
                    let (left, right) = dirs[1..dirs.len() - 1].split_once(", ").unwrap();
                    println!("{node} -> ({left}, {right})");
                    (node.into(), (left.into(), right.into()))
                })
                .collect::<HashMap<String, (String, String)>>(),
            instructions: CYCLE.chars().collect(),
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
}
