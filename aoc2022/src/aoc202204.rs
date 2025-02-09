use std::ops::RangeInclusive;

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    pairs: Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>,
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
        (2022, 4)
    }

    fn parse(&mut self) {
        self.pairs = aoc::read_lines(&self.input)
            .iter()
            .map(|l| {
                let (left, right) = l.split_once(',').unwrap();
                (str_to_range(left), str_to_range(right))
            })
            .collect();
    }

    fn part1(&mut self) -> String {
        output(
            self.pairs
                .iter()
                .filter(|(left, right)| {
                    (left.contains(right.start()) && left.contains(right.end()))
                        || (right.contains(left.start()) && right.contains(left.end()))
                })
                .count(),
        )
    }

    fn part2(&mut self) -> String {
        output(
            self.pairs
                .iter()
                .filter(|(left, right)| {
                    left.contains(right.start()) || right.contains(left.start())
                })
                .count(),
        )
    }
}

fn str_to_range(s: &str) -> RangeInclusive<u8> {
    let (start, end) = s.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}
