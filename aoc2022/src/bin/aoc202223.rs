use std::{collections::HashSet, ops::RangeInclusive};

use aoc::{
    runner::{output, run_solution, Runner},
    Point,
};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day23.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    elf_locations: HashSet<Point<i64>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 23)
    }

    fn parse(&mut self) {
        self.elf_locations = aoc::read_grid(&self.input)
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(c, ch)| {
                        if ch == &'#' {
                            Some(Point(r as i64, c as i64))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        if cfg!(test) {
            let bounds = self.bounds();
            for row in bounds.0 {
                for col in bounds.1.clone() {
                    print!(
                        "{}",
                        match self.elf_locations.get(&Point(row, col)) {
                            Some(_) => '#',
                            None => '.',
                        }
                    )
                }
                println!();
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl AocDay {
    fn bounds(&self) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
        let mut row_range = (i64::MAX, i64::MIN);
        let mut col_range = (i64::MAX, i64::MIN);
        for loc in &self.elf_locations {
            row_range.0 = row_range.0.min(loc.0);
            row_range.1 = row_range.1.max(loc.0);
            col_range.0 = col_range.0.min(loc.1);
            col_range.1 = col_range.1.max(loc.1);
        }
        (row_range.0..=row_range.1, col_range.0..=col_range.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 110;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 0;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
