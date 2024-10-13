use num::Integer;
use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub nodes: HashMap<String, (String, String)>,
    pub instructions: Vec<char>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
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

    fn part1(&mut self) -> String {
        output(self.find_cycle("AAA".into()))
    }

    fn part2(&mut self) -> String {
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
    pub fn find_cycle(&self, mut current: String) -> u64 {
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
