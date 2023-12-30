use std::{
    collections::{HashMap, HashSet},
    iter::Cycle,
    slice::Iter,
};

use aoc::runner::{output, run_solution, Runner};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day17.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    pattern: Vec<char>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 17)
    }

    fn parse(&mut self) {
        self.pattern = aoc::read_line(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut rocks = ROCKS.iter().cycle();
        let mut height = 0;
        let mut placed = HashSet::new();
        let mut pattern_index = 0;
        for _ in 0..2022 {
            let (h, _, idx, _) = self.drop_rock(height, &mut rocks, pattern_index, &mut placed);
            pattern_index = idx;
            height = height.max(h);
        }
        output(height)
    }

    fn part2(&mut self) -> Vec<String> {
        let total_drops = 1_000_000_000_000_usize;
        let mut rocks = ROCKS.iter().cycle();
        let mut height = 0;
        let mut placed = HashSet::new();
        let mut tower = vec![vec![false; 7]];
        let mut drop = 0;
        let mut drops = HashMap::new();
        let mut pattern_index = 0;
        let (start, cycle, start_height, cycle_height) = loop {
            drop += 1;
            let (h, rock, jet, points) =
                self.drop_rock(height, &mut rocks, pattern_index, &mut placed);
            pattern_index = jet;
            height = height.max(h);
            for _ in 0..(height - tower.len()) {
                tower.push(vec![false; 7]);
            }
            for p in points {
                tower[p.1][p.0] = true;
            }
            if height <= 4 {
                continue;
            }
            if let Some(prev) = drops.insert(
                (tower[(height - 4)..height].to_vec(), rock, jet),
                (drop, height),
            ) {
                break (prev.0, drop - prev.0, prev.1, height - prev.1);
            };
        };
        let cycles = (total_drops - start) / cycle;
        let height_at_cycle_end = start_height + cycles * cycle_height;
        let mut remaining_height = height;
        let remaining = (total_drops - start) % cycle;
        for _ in 0..remaining {
            let (h, _, idx, _) =
                self.drop_rock(remaining_height, &mut rocks, pattern_index, &mut placed);
            pattern_index = idx;
            remaining_height = remaining_height.max(h);
        }
        output(height_at_cycle_end + remaining_height - height)
    }
}

impl AocDay {
    fn drop_rock(
        &self,
        height: usize,
        rocks: &mut Cycle<Iter<Rock>>,
        mut pattern_index: usize,
        placed: &mut HashSet<(usize, usize)>,
    ) -> (usize, Rock, usize, HashSet<(usize, usize)>) {
        let cur = rocks.next().unwrap();
        let mut base = (2, height + 3);
        loop {
            // if height >
            let pat = self.pattern[pattern_index];
            base = cur.slide(pat, base, placed);
            pattern_index = (pattern_index + 1) % self.pattern.len();
            if let Some(shifted) = cur.drop(base, placed) {
                base = shifted;
            } else {
                let points = cur.points(base);
                placed.extend(points.clone());
                return (cur.top(base) + 1, *cur, pattern_index, points);
            };
        }
    }
}

const ROCKS: [Rock; 5] = [
    Rock::Horizontal,
    Rock::Cross,
    Rock::Corner,
    Rock::Vertical,
    Rock::Square,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Rock {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Rock {
    fn points(&self, base: (usize, usize)) -> HashSet<(usize, usize)> {
        let x = base.0;
        let y = base.1;
        match self {
            Rock::Horizontal => HashSet::from([base, (x + 1, y), (x + 2, y), (x + 3, y)]),
            Rock::Cross => HashSet::from([
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 1),
            ]),
            Rock::Corner => HashSet::from([
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ]),
            Rock::Vertical => HashSet::from([base, (x, y + 1), (x, y + 2), (x, y + 3)]),
            Rock::Square => HashSet::from([base, (x + 1, y), (x, y + 1), (x + 1, y + 1)]),
        }
    }

    fn top(&self, base: (usize, usize)) -> usize {
        self.points(base).iter().map(|p| p.1).max().unwrap()
    }

    fn slide(
        &self,
        dir: char,
        base: (usize, usize),
        placed: &HashSet<(usize, usize)>,
    ) -> (usize, usize) {
        let points = self.points(base);
        let left = points.iter().map(|p| p.0).min().unwrap();
        let right = points.iter().map(|p| p.0).max().unwrap();

        // At a wall
        if (left == 0 && dir == '<') || (right == 6 && dir == '>') {
            return base;
        }

        let offset = match dir {
            '>' => 1,
            '<' => -1,
            _ => unreachable!("Unknown offset {dir}"),
        };
        let shifted = points
            .iter()
            .map(|p| ((p.0 as isize + offset) as usize, p.1))
            .collect::<HashSet<_>>();
        if placed.is_empty() || shifted.intersection(placed).count() == 0 {
            // Didn't hit anything.
            ((base.0 as isize + offset) as usize, base.1)
        } else {
            base
        }
    }

    fn drop(
        &self,
        base: (usize, usize),
        placed: &HashSet<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        let points = self.points(base);
        let bottom = points.iter().map(|p| p.1).min().unwrap();

        // At a wall
        if bottom == 0 {
            return None;
        }

        let shifted = points
            .iter()
            .map(|p| (p.0, p.1 - 1))
            .collect::<HashSet<_>>();
        if placed.is_empty() || shifted.intersection(placed).count() == 0 {
            // Didn't hit anything.
            Some((base.0, base.1 - 1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 3068;
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
        let expected = 1_514_285_714_288_usize;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
