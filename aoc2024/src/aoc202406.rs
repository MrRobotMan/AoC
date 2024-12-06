use std::collections::{HashMap, HashSet};

use aoc::{
    read_grid,
    runner::{output, Runner},
    Point, CARDINALS,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    lab: HashMap<Point<i64>, char>,
    start: Point<i64>,
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
        (2024, 6)
    }

    fn parse(&mut self) {
        for (r, row) in read_grid(&self.input).iter().enumerate() {
            for (c, ch) in row.iter().enumerate() {
                match ch {
                    '^' => {
                        self.start = Point(r as i64, c as i64);
                        self.lab.insert(Point(r as i64, c as i64), '.');
                    }
                    _ => {
                        self.lab.insert(Point(r as i64, c as i64), *ch);
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> String {
        let mut visited = HashSet::new();
        let mut turns = CARDINALS.iter().cycle();
        let mut dir = turns.next().unwrap();
        let mut cur = self.start;
        while let Some(p) = self.lab.get(&cur) {
            match p {
                '#' => {
                    cur -= *dir; // Back up.
                    dir = turns.next().unwrap();
                }
                _ => {
                    visited.insert(cur);
                    cur += *dir;
                }
            }
        }
        output(visited.len())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        );
        day.parse();
        let expected = "41";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
