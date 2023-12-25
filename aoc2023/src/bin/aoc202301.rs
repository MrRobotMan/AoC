use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, run_solution, Runner},
};

fn main() {
    let mut day = AocDay {
        input: "inputs/day01.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    calibrations: Vec<String>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 1)
    }

    fn parse(&mut self) {
        self.calibrations = read_lines(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        let result = restore_calibrations(
            &self.calibrations,
            &HashMap::from([
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9),
            ]),
        );
        output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let result = restore_calibrations(
            &self.calibrations,
            &HashMap::from([
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9),
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]),
        );
        output(result)
    }
}

fn restore_calibrations(calibrations: &[String], search_space: &HashMap<&str, u64>) -> u64 {
    calibrations
        .iter()
        .map(|line| convert(line, search_space))
        .sum()
}

fn convert(calibration: &str, search_space: &HashMap<&str, u64>) -> u64 {
    let mut first = (calibration.len(), 0);
    let mut last = (0, 0);
    for (key, num) in search_space.iter() {
        if let Some(start) = calibration.find(key) {
            if start < first.0 {
                first = (start, *num);
            }
        };
        if let Some(end) = calibration.rfind(key) {
            if end >= last.0 {
                last = (end, *num);
            }
        };
    }
    first.1 * 10 + last.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 142;
        let actual = restore_calibrations(
            &[
                "1abc2".into(),
                "pqr3stu8vwx".into(),
                "a1b2c3d4e5f".into(),
                "treb7uchet".into(),
            ],
            &HashMap::from([
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9),
            ]),
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_part2() {
        let expected = 281;
        let search_space = HashMap::from([
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);
        let converted = [
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];
        let actual = restore_calibrations(&converted, &search_space);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_convert() {
        let search_space = HashMap::from([
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);
        let expected = 83;
        let actual = convert("eightwothree", &search_space);
        assert_eq!(expected, actual);
    }
}
