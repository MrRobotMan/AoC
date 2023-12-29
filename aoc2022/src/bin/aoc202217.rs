use std::collections::HashSet;

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
        let mut pattern = self.pattern.iter().cycle();
        let mut rocks = ROCKS.iter().cycle();
        let mut height = 3;
        let mut placed = HashSet::new();
        'next_rock: for _ in 0..2022 {
            let cur = rocks.next().unwrap();
            let mut base = (2, height);
            // _show(&cur.points(base), &placed, height);
            loop {
                base = cur.slide(*pattern.next().unwrap(), base, &placed);
                if let Some(shifted) = cur.drop(base, &placed) {
                    base = shifted;
                } else {
                    placed.extend(cur.points(base));
                    height = height.max(cur.top(base) + 4);
                    continue 'next_rock;
                };
            }
        }
        output(height - 3)
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

fn _show(cur: &HashSet<(isize, isize)>, placed: &HashSet<(isize, isize)>, height: isize) {
    for row in 0..=height {
        print!("|");
        for col in 0..7 {
            if cur.contains(&(col, height - row)) {
                print!("@");
            } else if placed.contains(&(col, height - row)) {
                print!("#");
            } else {
                print!(".");
            };
        }
        println!("|");
    }
    println!("+-------+\n");
}

const ROCKS: [Rock; 5] = [
    Rock::Horizontal,
    Rock::Cross,
    Rock::Corner,
    Rock::Vertical,
    Rock::Square,
];

enum Rock {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Rock {
    fn points(&self, base: (isize, isize)) -> HashSet<(isize, isize)> {
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

    fn top(&self, base: (isize, isize)) -> isize {
        self.points(base).iter().map(|p| p.1).max().unwrap()
    }

    fn slide(
        &self,
        dir: char,
        base: (isize, isize),
        placed: &HashSet<(isize, isize)>,
    ) -> (isize, isize) {
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
            .map(|p| (p.0 + offset, p.1))
            .collect::<HashSet<_>>();
        if placed.is_empty() || shifted.intersection(placed).count() == 0 {
            // Didn't hit anything.
            (base.0 + offset, base.1)
        } else {
            base
        }
    }

    fn drop(
        &self,
        base: (isize, isize),
        placed: &HashSet<(isize, isize)>,
    ) -> Option<(isize, isize)> {
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
        let expected = 0;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
