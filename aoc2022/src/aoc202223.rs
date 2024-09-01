use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use aoc::{
    runner::{output, Runner},
    Point,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    pub(crate) elf_locations: HashSet<Point<i64>>,
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
    }

    fn part1(&mut self) -> Vec<String> {
        for cur in 0..10 {
            self.round(cur);
        }
        let (rows, cols) = self.bounds();
        output(
            (rows.end() - rows.start() + 1) * (cols.end() - cols.start() + 1)
                - self.elf_locations.len() as i64,
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut rounds = 10; // Aleady done in part 1.
        loop {
            if !self.round(rounds) {
                break;
            }
            rounds += 1;
        }
        output(rounds + 1)
    }
}

// Dirs are N, S, W, E each indicating with 3 of the 8 surrounding cells to check.
// 0 1 2
// 7 x 3
// 6 5 4
const DIRS: [[usize; 3]; 4] = [[0, 1, 2], [4, 5, 6], [6, 7, 0], [2, 3, 4]];
const DIR: [Point<i64>; 4] = [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];

impl AocDay {
    fn round(&mut self, cur: usize) -> bool {
        let mut proposed: HashMap<Point<i64>, Vec<Point<i64>>> = HashMap::new();
        for elf in &self.elf_locations {
            if let Some(loc) = self.propose(*elf, cur) {
                proposed
                    .entry(loc)
                    .and_modify(|v| v.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }
        if proposed.is_empty() {
            return false;
        }
        for (moved, elves) in proposed {
            if elves.len() == 1 {
                for elf in elves {
                    self.elf_locations.remove(&elf);
                }
                self.elf_locations.insert(moved);
            }
        }
        true
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

    fn propose(&self, elf: Point<i64>, dir: usize) -> Option<Point<i64>> {
        let surrounding = self.surrounding(elf);
        if surrounding.iter().all(|c| *c) {
            return None;
        }
        for idx in 0..4 {
            let indices = DIRS[(idx + dir) % 4];
            if surrounding[indices[0]] && surrounding[indices[1]] && surrounding[indices[2]] {
                return Some(elf + DIR[(idx + dir) % 4]);
            }
        }

        None
    }

    /// Check the 8 surrounding cells. Returning true if empty and false if occupied.
    ///  0 1 2
    ///  7 x 3
    ///  6 5 4
    fn surrounding(&self, elf: Point<i64>) -> [bool; 8] {
        [
            self.elf_locations.contains(&(elf + Point(-1, -1))),
            self.elf_locations.contains(&(elf + Point(-1, 0))),
            self.elf_locations.contains(&(elf + Point(-1, 1))),
            self.elf_locations.contains(&(elf + Point(0, 1))),
            self.elf_locations.contains(&(elf + Point(1, 1))),
            self.elf_locations.contains(&(elf + Point(1, 0))),
            self.elf_locations.contains(&(elf + Point(1, -1))),
            self.elf_locations.contains(&(elf + Point(0, -1))),
        ]
    }
}
