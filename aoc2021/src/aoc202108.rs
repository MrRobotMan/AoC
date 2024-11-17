use aoc::{
    read_lines,
    runner::{output, Runner},
};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    display: Vec<Segment>,
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
        (2021, 8)
    }

    fn parse(&mut self) {
        self.display = read_lines(&self.input).iter().map(Segment::from).collect();
    }

    fn part1(&mut self) -> String {
        output(self.display.iter().map(|d| d.count_unique()).sum::<usize>())
    }

    fn part2(&mut self) -> String {
        for display in self.display.iter_mut() {
            display.translate();
        }
        output(self.display.iter().map(|d| d.decode()).sum::<usize>())
    }
}

const PATTERNS: [&str; 10] = [
    "abcefg",  // 0
    "cf",      // 1
    "acdeg",   // 2
    "acdfg",   // 3
    "bcdf",    // 4
    "abdfg",   // 5
    "abdefg",  // 6
    "acf",     // 7
    "abcdefg", // 8
    "abcdfg",  // 9
];

#[derive(Debug, Default)]
struct Segment {
    signal_patterns: Vec<String>,
    target_value: Vec<String>,
    translation_matrix: HashMap<char, char>,
}

impl Segment {
    fn count_unique(&self) -> usize {
        self.target_value
            .iter()
            .filter(|v| [2, 3, 4, 7].contains(&v.len()))
            .count()
    }

    fn decode(&self) -> usize {
        let mut res = 0;
        for val in &self.target_value {
            let mut trans = val
                .chars()
                .map(|c| self.translation_matrix[&c])
                .collect::<Vec<_>>();
            trans.sort();
            let repr = trans.into_iter().collect::<String>();
            res += PATTERNS.iter().position(|p| **p == repr).unwrap();
            res *= 10;
        }
        res / 10
    }

    fn translate(&mut self) {
        let one = get_groups(&self.signal_patterns, 2).pop().unwrap();
        let four = get_groups(&self.signal_patterns, 4).pop().unwrap();
        let seven = get_groups(&self.signal_patterns, 3).pop().unwrap();
        let two_three_five = get_groups(&self.signal_patterns, 5);
        let a = *seven.difference(&one).next().unwrap();
        let cf = one.clone();
        let bd = four.difference(&one).copied().collect::<HashSet<_>>();
        let two = two_three_five
            .iter()
            .find(|seq| {
                cf.difference(seq).collect::<Vec<_>>().len() == 1
                    && bd.difference(seq).collect::<Vec<_>>().len() == 1
            })
            .unwrap();
        let three = two_three_five
            .iter()
            .find(|seq| {
                cf.difference(seq).collect::<Vec<_>>().is_empty()
                    && bd.difference(seq).collect::<Vec<_>>().len() == 1
            })
            .unwrap();
        let five = two_three_five
            .iter()
            .find(|seq| {
                cf.difference(seq).collect::<Vec<_>>().len() == 1
                    && bd.difference(seq).collect::<Vec<_>>().is_empty()
            })
            .unwrap();
        let mut eg = two.difference(&four).copied().collect::<HashSet<_>>();
        eg.remove(&a);
        let e = *two.difference(three).next().unwrap();
        eg.remove(&e);
        let g = *eg.iter().next().unwrap();
        let c = *cf.difference(five).next().unwrap();
        let f = *cf.difference(two).next().unwrap();
        let b = *bd.difference(three).next().unwrap();
        let d = *bd.difference(&HashSet::from([b])).next().unwrap();
        self.translation_matrix = HashMap::from([
            (a, 'a'),
            (b, 'b'),
            (c, 'c'),
            (d, 'd'),
            (e, 'e'),
            (f, 'f'),
            (g, 'g'),
        ]);
    }
}

fn get_groups(patterns: &[String], length: usize) -> Vec<HashSet<char>> {
    patterns
        .iter()
        .filter_map(|v| {
            if v.len() == length {
                Some(v.chars().collect())
            } else {
                None
            }
        })
        .collect()
}

impl From<&String> for Segment {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}
impl From<&str> for Segment {
    fn from(value: &str) -> Self {
        let (input, output) = value.split_once("|").unwrap();
        let signal_patterns = input.split_whitespace().map(String::from).collect();
        let target_value = output.split_whitespace().map(String::from).collect();
        Self {
            signal_patterns,
            target_value,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example1() {
        let mut segment: Segment =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
                .into();
        segment.translate();
        assert_eq!(
            HashMap::from([
                ('d', 'a'),
                ('e', 'b'),
                ('a', 'c'),
                ('f', 'd'),
                ('g', 'e'),
                ('b', 'f'),
                ('c', 'g'),
            ]),
            segment.translation_matrix
        );
        assert_eq!(5353, segment.decode());
    }
}
