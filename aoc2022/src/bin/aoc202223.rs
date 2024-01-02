use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

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
    initial_layout: HashSet<Point<i64>>,
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

        self.initial_layout = self.elf_locations.clone();
        if cfg!(test) {
            self.show();
        };
    }

    fn part1(&mut self) -> Vec<String> {
        for cur in 0..10 {
            let mut proposed: HashMap<Point<i64>, Vec<Point<i64>>> = HashMap::new();
            for elf in &self.elf_locations {
                let loc = self.propose(*elf, cur);
                proposed
                    .entry(loc)
                    .and_modify(|v| v.push(*elf))
                    .or_insert(vec![*elf]);
            }
            for (moved, elves) in proposed {
                if elves.len() == 1 {
                    for elf in elves {
                        self.elf_locations.remove(&elf);
                    }
                    self.elf_locations.insert(moved);
                }
            }
        }
        let (rows, cols) = self.bounds();
        output(
            (rows.end() - rows.start() + 1) * (cols.end() - cols.start() + 1)
                - self.elf_locations.len() as i64,
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

// Dirs are N, S, W, E each indicating with 3 of the 8 surrounding cells to check.
// 0 1 2
// 7 x 3
// 6 5 4
const DIRS: [[usize; 3]; 4] = [[0, 1, 2], [4, 5, 6], [6, 7, 0], [2, 3, 4]];
const DIR: [Point<i64>; 4] = [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];

impl AocDay {
    fn show(&self) {
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

    fn propose(&self, elf: Point<i64>, dir: usize) -> Point<i64> {
        let surrounding = self.surrounding(elf);
        if surrounding.iter().all(|c| *c) {
            return elf;
        }
        for idx in 0..4 {
            let indices = DIRS[(idx + dir) % 4];
            if surrounding[indices[0]] && surrounding[indices[1]] && surrounding[indices[2]] {
                return elf + DIR[(idx + dir) % 4];
            }
        }

        elf
    }

    /// Check the 8 surrounding cells. Returning true if empty and false if occupied.
    ///  0 1 2
    ///  7 x 3
    ///  6 5 4
    fn surrounding(&self, elf: Point<i64>) -> [bool; 8] {
        [
            self.elf_locations.get(&(elf + Point(-1, -1))).is_none(),
            self.elf_locations.get(&(elf + Point(-1, 0))).is_none(),
            self.elf_locations.get(&(elf + Point(-1, 1))).is_none(),
            self.elf_locations.get(&(elf + Point(0, 1))).is_none(),
            self.elf_locations.get(&(elf + Point(1, 1))).is_none(),
            self.elf_locations.get(&(elf + Point(1, 0))).is_none(),
            self.elf_locations.get(&(elf + Point(1, -1))).is_none(),
            self.elf_locations.get(&(elf + Point(0, -1))).is_none(),
        ]
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
