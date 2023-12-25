use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, run_solution, Runner},
};

fn main() {
    let mut day = AocDay {
        input: "inputs/day03.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Debug, Default, PartialEq)]
struct AocDay {
    input: String,
    part_numbers: HashMap<(usize, usize), String>,
    symbols: HashMap<(usize, usize), char>,
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

    fn part1(&mut self) -> Vec<String> {
        output(self.get_part_total())
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.get_gear_ratios())
    }
}

impl AocDay {
    fn get_part_total(&self) -> u32 {
        self.part_numbers
            .iter()
            .map(|(loc, value)| {
                if offsets(loc, value.len())
                    .iter()
                    .any(|l| self.symbols.get(l).is_some())
                {
                    value.parse::<u32>().unwrap()
                } else {
                    0
                }
            })
            .sum()
    }
    fn get_gear_ratios(&self) -> u32 {
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        for (loc, value) in &self.part_numbers {
            for l in offsets(loc, value.len()) {
                if let Some(c) = self.symbols.get(&l) {
                    if c == &'*' {
                        let val = value.parse::<u32>().unwrap();
                        if let Some(entry) = gears.get_mut(&l) {
                            entry.push(val);
                        } else {
                            gears.insert(l, vec![val]);
                        }
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

fn parse_string(lines: Vec<String>) -> Mapping {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let lines = vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];
        let mut actual = AocDay::default();
        (actual.part_numbers, actual.symbols) = parse_string(lines);
        let expected = AocDay {
            part_numbers: HashMap::from([
                ((0, 0), "467".into()),
                ((0, 5), "114".into()),
                ((2, 2), "35".into()),
                ((2, 6), "633".into()),
                ((4, 0), "617".into()),
                ((5, 7), "58".into()),
                ((6, 2), "592".into()),
                ((7, 6), "755".into()),
                ((9, 1), "664".into()),
                ((9, 5), "598".into()),
            ]),
            symbols: HashMap::from([
                ((1, 3), '*'),
                ((3, 6), '#'),
                ((4, 3), '*'),
                ((5, 5), '+'),
                ((8, 3), '$'),
                ((8, 5), '*'),
            ]),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_line_ending_value() {
        let lines = vec![
            "...500$401".into(),
            "..35..40.*".into(),
            "67........".into(),
        ];
        let mut actual = AocDay::default();
        (actual.part_numbers, actual.symbols) = parse_string(lines);
        let expected = AocDay {
            part_numbers: HashMap::from([
                ((0, 3), "500".into()),
                ((0, 7), "401".into()),
                ((1, 2), "35".into()),
                ((1, 6), "40".into()),
                ((2, 0), "67".into()),
            ]),
            symbols: HashMap::from([((0, 6), '$'), ((1, 9), '*')]),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let expected = 4361;
        let lines = vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];
        let mut actual = AocDay::default();
        (actual.part_numbers, actual.symbols) = parse_string(lines);
        let actual = actual.get_part_total();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_part2() {
        let expected = 467835;
        let lines = vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];
        let mut actual = AocDay::default();
        (actual.part_numbers, actual.symbols) = parse_string(lines);
        let actual = actual.get_gear_ratios();
        assert_eq!(expected, actual);
    }
}
