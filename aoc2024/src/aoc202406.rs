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
    nodes_walked: HashSet<Point<i64>>,
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
        self.nodes_walked = self.walk().unwrap().iter().map(|(p, _)| *p).collect();
        output(self.nodes_walked.len())
    }

    fn part2(&mut self) -> String {
        let mut valid_new_obstacle = 0;
        for node in &self.nodes_walked {
            if *node == self.start {
                continue;
            }
            self.lab.entry(*node).and_modify(|ch| *ch = '#');
            if self.walk().is_none() {
                valid_new_obstacle += 1;
            }
            self.lab.entry(*node).and_modify(|ch| *ch = '.');
        }
        output(valid_new_obstacle)
    }
}

impl AocDay {
    fn walk(&self) -> Option<HashSet<(Point<i64>, Point<i64>)>> {
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
                    if !visited.insert((cur, *dir)) {
                        return None; // Found a loop
                    };
                    cur += *dir;
                }
            }
        }
        Some(visited)
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

    #[test]
    fn test_example2() {
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
        day.part1();
        let expected = "6";
        let actual = day.part2();
        assert_eq!(expected, actual);
    }
}
