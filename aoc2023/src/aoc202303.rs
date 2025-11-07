use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Debug, Default, PartialEq)]
pub struct AocDay {
    pub input: String,
    pub part_numbers: HashMap<(usize, usize), String>,
    pub symbols: HashMap<(usize, usize), char>,
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
        (2023, 3)
    }

    fn parse(&mut self) {
        let (parts, symbols) = parse_string(read_lines(&self.input));
        self.part_numbers = parts;
        self.symbols = symbols;
    }

    fn part1(&mut self) -> String {
        output(self.get_part_total())
    }

    fn part2(&mut self) -> String {
        output(self.get_gear_ratios())
    }
}

impl AocDay {
    pub fn get_part_total(&self) -> u32 {
        self.part_numbers
            .iter()
            .map(|(loc, value)| {
                if offsets(loc, value.len())
                    .iter()
                    .any(|l| self.symbols.contains_key(l))
                {
                    value.parse::<u32>().unwrap()
                } else {
                    0
                }
            })
            .sum()
    }
    pub fn get_gear_ratios(&self) -> u32 {
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        for (loc, value) in &self.part_numbers {
            for l in offsets(loc, value.len()) {
                if let Some(c) = self.symbols.get(&l)
                    && c == &'*' {
                        let val = value.parse::<u32>().unwrap();
                        if let Some(entry) = gears.get_mut(&l) {
                            entry.push(val);
                        } else {
                            gears.insert(l, vec![val]);
                        }
                    }
            }
        }
        gears
            .values()
            .filter_map(|v| {
                if v.len() == 2 {
                    Some(v[0] * v[1])
                } else {
                    None
                }
            })
            .sum()
    }
}

fn offsets(loc: &(usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut locs = Vec::new();
    for col in loc.1.saturating_sub(1)..=loc.1 + size {
        locs.push((loc.0.saturating_sub(1), col));
        locs.push((loc.0, col));
        locs.push((loc.0 + 1, col));
    }
    locs
}

type Mapping = (
    HashMap<(usize, usize), String>,
    HashMap<(usize, usize), char>,
);

pub fn parse_string(lines: Vec<String>) -> Mapping {
    let mut parts = HashMap::new();
    let mut symbols = HashMap::new();
    let mut current: Option<String> = None;
    for (row, line) in lines.iter().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            match chr {
                c if c.is_ascii_digit() => {
                    if let Some(ref mut val) = current {
                        val.push(c);
                    } else {
                        current = Some(String::from(c));
                    }
                }
                c => {
                    if let Some(val) = current.take() {
                        parts.insert((row, col - val.len()), val);
                    }
                    if c != '.' {
                        symbols.insert((row, col), c);
                    };
                }
            }
        }
        if let Some(val) = current.take() {
            parts.insert((row, line.len() - val.len()), val);
        }
        current = None;
    }
    (parts, symbols)
}
