use std::{
    num::ParseIntError,
    ops::{Add, Sub},
    str::FromStr,
};

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day22.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    bricks: Vec<Brick>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 22)
    }

    fn parse(&mut self) {
        self.bricks = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.parse().unwrap())
            .collect();

        // Make sure all bricks are orthogonal. Plan doesn't work if they're tilted.
        assert_eq!(0, self.bricks.iter().filter(|b| !b.is_orthogonal()).count());
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, Default)]
struct Brick {
    start: Point,
    end: Point,
    supporting: Vec<i64>,
    supported: Vec<i64>,
}

impl Brick {
    fn delta(&self) -> (i64, i64, i64) {
        self.end - self.start
    }

    fn length(&self) -> i64 {
        let (dx, dy, dz) = self.delta();
        (dx + dy + dz).abs() + 1
    }

    fn is_orthogonal(&self) -> bool {
        let (dx, dy, dz) = self.delta();
        [dx != 0, dy != 0, dz != 0]
            .iter()
            .map(|b| if *b { 0 } else { 1 })
            .sum::<usize>()
            > 1
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
}
