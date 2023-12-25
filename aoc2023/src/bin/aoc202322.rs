use std::{
    num::ParseIntError,
    ops::{Add, Range, Sub},
    str::FromStr,
};

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/day22.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    bricks: Vec<Brick>,
    plane: (i64, i64, i64, i64),
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 22)
    }

    fn parse(&mut self) {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        for line in aoc::read_lines(&self.input) {
            let brick: Brick = line.parse().unwrap();
            min_x = min_x.min(brick.start.0).min(brick.end.0);
            max_x = max_x.max(brick.start.0).max(brick.end.0);
            min_y = min_y.min(brick.start.1).min(brick.end.1);
            max_y = max_y.max(brick.start.1).max(brick.end.1);
            self.bricks.push(brick);
        }
        self.plane = (min_x, max_x, min_y, max_y);
        // Make sure all bricks are orthogonal. Plan doesn't work if they're tilted.
        assert_eq!(0, self.bricks.iter().filter(|b| !b.is_orthogonal()).count());
        self.bricks.sort_by_key(|brick| brick.lowest());
        for i in 0..self.bricks.len() {
            place_brick(i, &mut self.bricks);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let disintigratable = self
            .bricks
            .iter()
            .filter(|b| {
                b.supporting.is_empty()
                    || b.supporting
                        .iter()
                        .all(|a| self.bricks[*a as usize].supported_by.len() > 1)
            })
            .count();
        output(disintigratable)
    }

    fn part2(&mut self) -> Vec<String> {
        let canditates = self
            .bricks
            .iter()
            .enumerate()
            .filter_map(|(idx, b)| {
                if !b.supporting.is_empty()
                    && b.supporting
                        .iter()
                        .any(|a| self.bricks[*a as usize].supported_by.len() <= 1)
                {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let count = canditates.iter().fold(0, |acc, idx| {
            let mut bricks = self.bricks.clone();
            bricks.remove(*idx);
            acc + (0..bricks.len()).fold(0, |t, i| {
                if place_brick(i, &mut bricks) {
                    t + 1
                } else {
                    t
                }
            })
        });
        output(count)
    }
}

fn place_brick(idx: usize, bricks: &mut [Brick]) -> bool {
    let deltas = bricks[idx].coordinates();
    let dx = &deltas[0];
    let dy = &deltas[1];
    let mut z = 0;
    let mut canditates = Vec::new();
    for (i, b) in bricks[..idx].iter().enumerate() {
        if b.overlaps(dx, 0) && b.overlaps(dy, 1) {
            canditates.push((i, b.highest()));
            z = z.max(b.highest());
        }
    }
    let mut supporting = Vec::new();
    for (i, brick) in bricks[..=idx].iter_mut().enumerate() {
        if canditates.contains(&(i, z)) {
            brick.supporting.push(idx as i64);
            supporting.push(i as i64);
        }
        if i == idx {
            brick.supported_by.extend(supporting.clone());
            if (z + 1) == brick.lowest() {
                return false;
            };
            brick.shift_down_to(z + 1);
        }
    }
    true
}

#[derive(Debug, Default, Clone)]
struct Brick {
    start: Point,
    end: Point,
    supporting: Vec<i64>,
    supported_by: Vec<i64>,
}

impl Brick {
    fn lowest(&self) -> i64 {
        self.start.2.min(self.end.2)
    }

    fn highest(&self) -> i64 {
        self.start.2.max(self.end.2)
    }

    fn shift_down_to(&mut self, elevation: i64) {
        let dz = self.lowest() - elevation;
        self.start.2 -= dz;
        self.end.2 -= dz;
    }

    fn delta(&self) -> [i64; 3] {
        (self.end - self.start).into()
    }

    fn overlaps(&self, other: &Range<i64>, idx: usize) -> bool {
        let d = &self.coordinates()[idx];
        d.start <= other.end && d.end >= other.start
    }

    fn coordinates(&self) -> [Range<i64>; 3] {
        [
            self.start.0.min(self.end.0)..self.start.0.max(self.end.0),
            self.start.1.min(self.end.1)..self.start.1.max(self.end.1),
            self.start.2.min(self.end.2)..self.start.2.max(self.end.2),
        ]
    }

    fn _length(&self) -> i64 {
        self.delta().iter().sum::<i64>().abs() + 1
    }

    fn is_orthogonal(&self) -> bool {
        self.delta().iter().filter(|d| **d != 0).count() <= 1
    }
}

impl FromStr for Brick {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct Point(i64, i64, i64);

impl Sub for Point {
    type Output = (i64, i64, i64);

    fn sub(self, rhs: Self) -> Self::Output {
        (rhs.0 - self.0, rhs.1 - self.1, rhs.2 - self.2)
    }
}

impl Add for Point {
    type Output = (i64, i64, i64);

    fn add(self, rhs: Self) -> Self::Output {
        (rhs.0 + self.0, rhs.1 + self.1, rhs.2 + self.2)
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|p| p.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(parts[0], parts[1], parts[2]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 5;
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
        let expected = 7;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
