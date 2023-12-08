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
                println!("{node} -> ({left}, {right})");
                (node.into(), (left.into(), right.into()))
            })
            .collect::<HashMap<String, (String, String)>>();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
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
}
