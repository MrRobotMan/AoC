use aoc::runner::{output, Runner};
use std::collections::HashSet;

#[derive(Default)]
pub struct AocDay {
    input: String,
    instructions: Vec<(String, i32)>,
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
        (2022, 9)
    }

    fn parse(&mut self) {
        self.instructions = aoc::read_lines(&self.input)
            .into_iter()
            .map(|line| {
                let (left, right) = line.split_once(' ').unwrap();
                (left.to_string(), right.parse().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut head = Knot::default();
        let mut tail = Knot::default();
        for (direction, qty) in &self.instructions {
            for _ in 0..*qty {
                head.move_to(direction);
                process_head_tail(&head, &mut tail);
            }
        }
        output(tail.visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut knots = vec![Knot::default(); 10];
        for (direction, qty) in &self.instructions {
            for _ in 0..*qty {
                knots[0].move_to(direction);
                for idx in 1..knots.len() {
                    let head = &knots[idx - 1].clone();
                    process_head_tail(head, &mut knots[idx]);
                }
            }
        }

        output(knots.last().unwrap().visited.len())
    }
}

#[derive(Debug, Clone)]
struct Knot {
    current: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Default for Knot {
    fn default() -> Self {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));
        Self {
            current: Default::default(),
            visited,
        }
    }
}

impl Knot {
    fn move_to(&mut self, direction: &str) {
        let (mut row, mut col) = self.current;
        for chr in direction.chars() {
            match chr {
                'U' => row += 1,
                'D' => row -= 1,
                'L' => col -= 1,
                'R' => col += 1,
                _ => (),
            }
        }
        self.current = (row, col);
        self.visited.insert(self.current);
    }

    fn distance(&self, other: &Self) -> (i32, i32) {
        (
            self.current.0 - other.current.0,
            self.current.1 - other.current.1,
        )
    }
}

fn process_head_tail(head: &Knot, tail: &mut Knot) {
    match head.distance(tail) {
        (2, 0) => tail.move_to("U"),
        (2, 1) | (2, 2) | (1, 2) => tail.move_to("UR"),
        (0, 2) => tail.move_to("R"),
        (-1, 2) | (-2, 2) | (-2, 1) => tail.move_to("DR"),
        (-2, 0) => tail.move_to("D"),
        (-2, -1) | (-2, -2) | (-1, -2) => tail.move_to("DL"),
        (0, -2) => tail.move_to("L"),
        (1, -2) | (2, -2) | (2, -1) => tail.move_to("UL"),
        _ => (),
    }
}
