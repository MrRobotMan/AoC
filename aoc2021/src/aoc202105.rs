use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    vent_lines: Vec<((u32, u32), (u32, u32))>,
    vents: HashMap<(u32, u32), usize>,
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
        (2021, 5)
    }

    fn parse(&mut self) {
        self.vent_lines = read_lines(&self.input)
            .iter()
            .map(|line| {
                let (start, end) = line.split_once(" -> ").unwrap();
                (
                    start
                        .split_once(",")
                        .map(|v| (v.0.parse().unwrap(), v.1.parse().unwrap()))
                        .unwrap(),
                    end.split_once(",")
                        .map(|v| (v.0.parse().unwrap(), v.1.parse().unwrap()))
                        .unwrap(),
                )
            })
            .collect();
    }

    fn part1(&mut self) -> String {
        self.count_rows_and_columns();
        output(self.vents.values().filter(|v| **v > 1).count())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

impl AocDay {
    fn count_rows_and_columns(&mut self) {
        for (start, end) in &self.vent_lines {
            if start.0 == end.0 {
                let min = (start.1).min(end.1);
                let max = (start.1).max(end.1);
                for col in min..=max {
                    self.vents
                        .entry((start.0, col))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            } else if start.1 == end.1 {
                let min = (start.0).min(end.0);
                let max = (start.0).max(end.0);
                for row in min..=max {
                    self.vents
                        .entry((row, start.1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            vent_lines: vec![
                ((0, 9), (5, 9)),
                ((8, 0), (0, 8)),
                ((9, 4), (3, 4)),
                ((2, 2), (2, 1)),
                ((7, 0), (7, 4)),
                ((6, 4), (2, 0)),
                ((0, 9), (2, 9)),
                ((3, 4), (1, 4)),
                ((0, 0), (8, 8)),
                ((5, 5), (8, 2)),
            ],
            ..Default::default()
        };
        assert_eq!("5", &day.part1());
    }
}
