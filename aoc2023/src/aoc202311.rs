use std::collections::HashSet;

use aoc::runner::{output, Runner};
use itertools::Itertools;

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub galaxies: HashSet<(i64, i64)>,
    pub cols: Vec<i64>,
    pub rows: Vec<i64>,
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
        (2023, 11)
    }

    fn parse(&mut self) {
        let lines = aoc::read_grid(&self.input);
        let mut galaxies = HashSet::new();
        self.cols = vec![1; lines[0].len()];
        self.rows = vec![1; lines.len()];
        for (row, line) in lines.into_iter().enumerate() {
            for (col, chr) in line.iter().enumerate() {
                if matches!(chr, '#') {
                    self.cols[col] = 0;
                    self.rows[row] = 0;
                    galaxies.insert((row as i64, col as i64));
                }
            }
        }
        self.galaxies = galaxies;
    }

    fn part1(&mut self) -> String {
        output(
            self.galaxies
                .iter()
                .combinations(2)
                .map(|combo| diff(&combo, 2, &self.cols, &self.rows))
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> String {
        output(
            self.galaxies
                .iter()
                .combinations(2)
                .map(|combo| diff(&combo, 1_000_000, &self.cols, &self.rows))
                .sum::<i64>(),
        )
    }
}

pub fn diff(points: &[&(i64, i64)], scale: i64, empty_cols: &[i64], empty_rows: &[i64]) -> i64 {
    let left = points[0].1.min(points[1].1);
    let right = points[0].1.max(points[1].1);
    let top = points[0].0.min(points[1].0);
    let bottom = points[0].0.max(points[1].0);
    let rows = top as usize..bottom as usize;
    let cols = left as usize..right as usize;
    right - left + bottom - top
        + (scale - 1)
            * (empty_rows[rows].iter().sum::<i64>() + empty_cols[cols].iter().sum::<i64>())
}
