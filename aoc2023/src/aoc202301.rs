use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    input: String,
    calibrations: Vec<String>,
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
        (2023, 1)
    }

    fn parse(&mut self) {
        self.calibrations = read_lines(&self.input);
    }

    fn part1(&mut self) -> String {
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

    fn part2(&mut self) -> String {
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

pub fn restore_calibrations(calibrations: &[String], search_space: &HashMap<&str, u64>) -> u64 {
    calibrations
        .iter()
        .map(|line| convert(line, search_space))
        .sum()
}

pub fn convert(calibration: &str, search_space: &HashMap<&str, u64>) -> u64 {
    let mut first = (calibration.len(), 0);
    let mut last = (0, 0);
    for (key, num) in search_space.iter() {
        if let Some(start) = calibration.find(key)
            && start < first.0 {
                first = (start, *num);
            };
        if let Some(end) = calibration.rfind(key)
            && end >= last.0 {
                last = (end, *num);
            };
    }
    first.1 * 10 + last.1
}
