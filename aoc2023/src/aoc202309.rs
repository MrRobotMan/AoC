use std::collections::VecDeque;

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub histories: Vec<History>,
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
        (2023, 9)
    }

    fn parse(&mut self) {
        self.histories = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.into())
            .collect();
    }

    fn part1(&mut self) -> String {
        let mut last = self.histories.clone();
        for hist in last.iter_mut() {
            hist.build_next();
        }
        output(
            last.iter()
                .fold(0, |acc, hist| acc + hist.values.back().unwrap()),
        )
    }

    fn part2(&mut self) -> String {
        let mut last = self.histories.clone();
        for hist in last.iter_mut() {
            hist.build_next();
        }
        output(
            last.iter()
                .fold(0, |acc, hist| acc + hist.values.front().unwrap()),
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct History {
    pub values: VecDeque<i64>,
}

impl History {
    pub fn build_next(&mut self) {
        self.values.make_contiguous();
        let mut temp = self.values.clone();
        let mut last = Vec::new();
        let mut first = Vec::new();
        while !zeroes(temp.as_slices().0) {
            last.push(*temp.back().unwrap());
            first.push(*temp.front().unwrap());
            temp = temp
                .as_slices()
                .0
                .windows(2)
                .map(|vals| vals[1] - vals[0])
                .collect();
        }
        last.reverse();
        first.reverse();
        self.values
            .push_back(last.into_iter().reduce(|acc, v| acc + v).unwrap());
        self.values
            .push_front(first.into_iter().reduce(|acc, v| v - acc).unwrap());
    }
}
fn zeroes(values: &[i64]) -> bool {
    values.iter().all(|v| v == &0)
}

impl From<&String> for History {
    fn from(value: &String) -> Self {
        let values = value
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Self { values }
    }
}
