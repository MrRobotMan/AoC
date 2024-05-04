use std::collections::HashSet;

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    items: Vec<String>,
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
        (2022, 3)
    }

    fn parse(&mut self) {
        self.items = aoc::read_lines(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.items.iter().fold(0, |acc, item| {
            let (first, second) = item.split_at(item.len() / 2);
            let first = first.chars().collect::<HashSet<_>>();
            let second = second.chars().collect::<HashSet<_>>();
            acc + first
                .intersection(&second)
                .fold(0, |acc, i| acc + get_item_priority(*i))
        }))
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.items.chunks(3).fold(0, |acc, sacks| {
            let s1 = sacks[0].chars().collect::<HashSet<_>>();
            let s2 = sacks[1].chars().collect::<HashSet<_>>();
            let s3 = sacks[2].chars().collect::<HashSet<_>>();
            acc + s1
                .intersection(&s2)
                .copied()
                .collect::<HashSet<_>>()
                // Intersection of the intersection of s1 & s2 with s1 & s3
                .intersection(&(s1.intersection(&s3).copied().collect::<HashSet<_>>()))
                .fold(0, |acc, i| acc + get_item_priority(*i))
        }))
    }
}

fn get_item_priority(item: char) -> u32 {
    let val = item as u32;
    if val > 96 {
        val - 96
    } else {
        val - 38
    }
}
