use std::collections::HashSet;

use aoc::runner::{output, run_solution, Runner};
use itertools::Itertools;

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day11.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    galaxies: HashSet<(i64, i64)>,
    cols: Vec<i64>,
    rows: Vec<i64>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 11)
    }

    fn parse(&mut self) {
        let lines = aoc::read_chars(&self.input);
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

    fn part1(&mut self) -> Vec<String> {
        output(
            self.galaxies
                .iter()
                .combinations(2)
                .map(|combo| diff(&combo, 2, &self.cols, &self.rows))
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output(
            self.galaxies
                .iter()
                .combinations(2)
                .map(|combo| diff(&combo, 1_000_000, &self.cols, &self.rows))
                .sum::<i64>(),
        )
    }
}

fn diff(points: &[&(i64, i64)], scale: i64, empty_cols: &[i64], empty_rows: &[i64]) -> i64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    #[test]
    fn test_parsing() {
        let expected = HashSet::from_iter([
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4),
        ]);
        let mut day = AocDay {
            input: INPUT.to_string(),
            ..Default::default()
        };

        day.parse();
        let actual = day.galaxies;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_distance() {
        let mut day = AocDay {
            input: INPUT.to_string(),
            ..Default::default()
        };
        day.parse();
        let expected = 9;
        let actual = diff(&[&(5, 1), &(9, 4)], 2, &day.cols, &day.rows);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let expected = 374;
        let mut day = AocDay {
            input: INPUT.to_string(),
            ..Default::default()
        };
        day.parse();
        let actual = day.part1()[0].parse::<i64>().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_small() {
        let expected = 1030;
        let mut day = AocDay {
            input: INPUT.to_string(),
            ..Default::default()
        };
        day.parse();
        let actual = day
            .galaxies
            .iter()
            .combinations(2)
            .map(|combo| diff(&combo, 10, &day.cols, &day.rows))
            .sum::<i64>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_lg() {
        let expected = 8410;
        let mut day = AocDay {
            input: INPUT.to_string(),
            ..Default::default()
        };
        day.parse();
        let actual = day
            .galaxies
            .iter()
            .combinations(2)
            .map(|combo| diff(&combo, 100, &day.cols, &day.rows))
            .sum::<i64>();
        assert_eq!(expected, actual);
    }
}
