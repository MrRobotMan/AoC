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
        let poly = (0..10).fold(self.polymer.clone(), |acc, _| acc.step());
        output(poly.most().1 - poly.least().1)
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug, Default, Clone)]
struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
    count: HashMap<char, usize>,
}

impl Polymer {
    fn step(&self) -> Self {
        let mut poly = Self {
            template: vec![self.template[0]],
            rules: self.rules.clone(),
            ..Default::default()
        };
        for pair in self.template.windows(2) {
            poly.template.push(self.rules[&(pair[0], pair[1])]);
            poly.template.push(pair[1]);
        }
        for chr in &poly.template {
            poly.count.entry(*chr).and_modify(|v| *v += 1).or_insert(1);
        }
        poly
    }

    fn most(&self) -> (&char, &usize) {
        self.count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap()
    }

    fn least(&self) -> (&char, &usize) {
        self.count.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap()
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
        let expected = &"NCNBCHB".chars().collect::<Vec<_>>();
        poly = poly.step();
        let actual = &poly.template;
        assert_eq!(expected, actual);
        for _ in 0..3 {
            poly = poly.step();
        }
        let expected = &"NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
            .chars()
            .collect::<Vec<_>>();
        let actual = &poly.template;
        assert_eq!(expected, actual);
        poly = (0..6).fold(poly, |acc, _| acc.step());
        assert_eq!(poly.most().1 - poly.least().1, 1588);
    }
}
