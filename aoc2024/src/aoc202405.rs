use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    must_proceed: HashMap<i32, Vec<i32>>,
    must_follow: HashMap<i32, Vec<i32>>,
    pages: Vec<Vec<i32>>,
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
        (2024, 5)
    }

    fn parse(&mut self) {
        for line in read_lines(&self.input) {
            match line.split_once('|') {
                Some((first, second)) => {
                    let first = first.parse::<i32>().unwrap();
                    let second = second.parse::<i32>().unwrap();
                    let proceed = self.must_proceed.entry(second).or_default();
                    proceed.push(first);
                    let follow = self.must_follow.entry(first).or_default();
                    follow.push(second);
                }
                None => self
                    .pages
                    .push(line.split(',').map(|num| num.parse().unwrap()).collect()),
            }
        }
    }

    fn part1(&mut self) -> String {
        output(
            self.pages
                .iter()
                .filter(|p| self.ordered(p))
                .map(|p| p[p.len() / 2])
                .sum::<i32>(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

impl AocDay {
    fn ordered(&self, pages: &[i32]) -> bool {
        for (idx, value) in pages.iter().enumerate() {
            let before = &pages[..idx];
            let after = &pages[idx + 1..];
            if let Some(req) = self.must_proceed.get(value) {
                if after.iter().any(|v| req.contains(v)) {
                    return false;
                }
            }
            if let Some(req) = self.must_follow.get(value) {
                if before.iter().any(|v| req.contains(v)) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
        );
        day.parse();
        let expected = "143";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
