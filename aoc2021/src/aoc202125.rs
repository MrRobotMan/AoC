use std::collections::HashMap;

use aoc::runner::{Runner, output};
use puzlib::{Vec2D, read_lines};

#[allow(dead_code)]
#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    herds: HashMap<Vec2D<usize>, SeaCucumber>,
    bounds: (usize, usize),
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
        (2021, 25)
    }

    fn parse(&mut self) {
        let lines = read_lines(&self.input);
        self.bounds = (lines.len(), lines[0].len());
        for (row, line) in lines.into_iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '>' => {
                        self.herds.insert(Vec2D(row, col), SeaCucumber::East);
                    }
                    'v' => {
                        self.herds.insert(Vec2D(row, col), SeaCucumber::South);
                    }
                    _ => (),
                }
            }
        }
    }

    fn part1(&mut self) -> String {
        let mut steps = 1;
        while self.step() {
            steps += 1;
        }
        output(steps)
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

impl AocDay {
    fn step(&mut self) -> bool {
        let mut moved = false;
        let mut next = HashMap::new();
        for (location, cucumber) in self
            .herds
            .iter()
            .filter(|c| matches!(c.1, SeaCucumber::East))
        {
            let mut next_location = *location + Vec2D(0, 1);
            if next_location.1 == self.bounds.1 {
                next_location.1 = 0;
            }
            if self.herds.contains_key(&next_location) {
                next.insert(*location, *cucumber);
            } else {
                next.insert(next_location, *cucumber);
                moved = true;
            }
        }
        self.herds.retain(|_, c| matches!(c, SeaCucumber::South));
        self.herds.extend(next.clone());
        for (location, cucumber) in self
            .herds
            .iter()
            .filter(|c| matches!(c.1, SeaCucumber::South))
        {
            let mut next_location = *location + Vec2D(1, 0);
            if next_location.0 == self.bounds.0 {
                next_location.0 = 0;
            }
            if self.herds.contains_key(&next_location) {
                next.insert(*location, *cucumber);
            } else {
                next.insert(next_location, *cucumber);
                moved = true;
            }
        }
        self.herds = next;
        moved
    }
}

#[derive(Debug, Clone, Copy)]
enum SeaCucumber {
    East,
    South,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 58;
        let mut day = AocDay::new(
            "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
        );
        day.parse();
        let actual = day.part1().parse().unwrap();
        assert_eq!(expected, actual);
    }
}
