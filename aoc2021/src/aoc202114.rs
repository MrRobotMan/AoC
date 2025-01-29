use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    polymer: Polymer,
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
        (2021, 14)
    }

    fn parse(&mut self) {
        self.polymer = read_lines(&self.input).into();
    }

    fn part1(&mut self) -> String {
        let mut poly = self.polymer.clone();
        poly.step(10);
        output(poly.delta())
    }

    fn part2(&mut self) -> String {
        let mut poly = self.polymer.clone();
        poly.step(40);
        output(poly.delta())
    }
}

#[derive(Debug, Default, Clone)]
struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
    count: HashMap<char, usize>,
    pairs: HashMap<(char, char), usize>,
}

impl Polymer {
    fn step(&mut self, count: usize) {
        for _ in 0..count {
            let mut pairs = HashMap::new();
            for ((left, right), cnt) in self.pairs.iter() {
                let insert = self.rules[&(*left, *right)];
                let v = pairs.entry((*left, insert)).or_default();
                *v += cnt;
                let v = pairs.entry((insert, *right)).or_default();
                *v += cnt;
            }
            self.pairs = pairs;
        }
    }

    fn init(&mut self) {
        self.count();
        for pair in self.template.windows(2) {
            self.pairs
                .entry((pair[0], pair[1]))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    fn count(&mut self) {
        let mut new_count = HashMap::new();
        for ((left, right), count) in &self.pairs {
            let l = new_count.entry(*left).or_default();
            *l += count;

            let r = new_count.entry(*right).or_default();
            *r += count;
        }
        // Counts everything twice except the first and last characters due to counting in pairs.
        new_count.entry(self.template[0]).and_modify(|v| *v += 1);
        new_count
            .entry(self.template[self.template.len() - 1])
            .and_modify(|v| *v += 1);
        for v in new_count.values_mut() {
            *v /= 2;
        }
        self.count = new_count;
    }

    fn most(&self) -> usize {
        *self
            .count
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap_or((&'_', &1))
            .1
    }

    fn least(&self) -> usize {
        *self
            .count
            .iter()
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap_or((&'_', &0))
            .1
    }

    fn delta(&mut self) -> usize {
        self.count();
        self.most() - self.least()
    }
}

impl From<Vec<String>> for Polymer {
    fn from(value: Vec<String>) -> Self {
        let mut polymer = Polymer::default();
        let mut iter = value.into_iter();
        polymer.template = iter.next().unwrap().chars().collect();
        for line in iter {
            let chars = line.chars().collect::<Vec<_>>();
            polymer
                .rules
                .insert((chars[0], chars[1]), chars[chars.len() - 1]);
        }
        polymer.init();
        polymer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut poly = Polymer {
            template: vec!['N', 'N', 'C', 'B'],
            rules: HashMap::from([
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ]),
            ..Default::default()
        };
        poly.init();
        poly.step(10);
        let delta = poly.delta();
        println!("\nCount\n{:?}", poly.count);
        assert_eq!(delta, 1588);
    }
}
