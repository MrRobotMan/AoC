use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    towels: Vec<String>,
    patterns: Vec<String>,
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
        (2024, 19)
    }

    fn parse(&mut self) {
        let lines = read_lines(&self.input);
        self.towels = lines[0].split(", ").map(|s| s.to_string()).collect();
        self.patterns = lines[1..].to_vec();
        self.towels.sort_by_key(|p| std::cmp::Reverse(p.len()));
    }

    fn part1(&mut self) -> String {
        output(
            self.patterns
                .iter()
                .filter(|p| {
                    let mut cache = Cache::default();
                    cache.can_design(p, &self.towels) > 0
                })
                .count(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug, Default)]
struct Cache {
    cache: HashMap<String, usize>,
}

impl Cache {
    fn can_design(&mut self, pattern: &str, towels: &[String]) -> usize {
        if pattern.is_empty() {
            return 1;
        }
        if let Some(count) = self.cache.get(pattern) {
            return *count;
        }
        let count = towels
            .iter()
            .filter_map(|towel| pattern.strip_prefix(towel))
            .map(|p| self.can_design(p, towels))
            .sum();
        self.cache.insert(pattern.into(), count);
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solvable() {
        let towels = "r, wr, b, g, bwu, rb, gb, br"
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let mut cache = Cache::default();
        let actual = cache.can_design("brwrr", &towels);
        assert!(actual > 0);
    }

    #[test]
    fn test_unsolvable() {
        let expected = 0;
        let towels = "r, wr, b, g, bwu, rb, gb, br"
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let mut cache = Cache::default();
        let actual = cache.can_design("ubwu", &towels);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "r, wr, b, g, bwu, rb, gb, br
brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );
        day.parse();
        println!("{:?}", day.towels);
        assert_eq!("6", day.part1());
    }
}
