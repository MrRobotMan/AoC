use std::collections::HashMap;

use aoc::{
    runner::{output, run_solution, Runner},
    Dir, Point,
};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day22.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    board: HashMap<Point<i64>, Cell>,
    instructions: Vec<Instruction>,
    width: i64,
    height: i64,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 22)
    }

    fn parse(&mut self) {
        let mut lines = aoc::read_lines(&self.input);
        let instructions = lines.pop().unwrap();
        self.height = lines.len() as i64;
        self.width = lines.iter().map(|r| r.len()).max().unwrap() as i64;

        self.board = lines
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(c, ch)| match ch {
                        '.' => Some((Point(r as i64, c as i64), Cell::Open)),
                        '#' => Some((Point(r as i64, c as i64), Cell::Solid)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut distance = 0;
        for ch in instructions.chars() {
            match ch.to_digit(10) {
                Some(v) => distance = distance * 10 + v as i64,
                None => {
                    let turn = match ch {
                        'R' => Turn::Right,
                        'L' => Turn::Left,
                        _ => panic!("Unknown direction {ch}"),
                    };
                    self.instructions.push(Instruction::Distance(distance));
                    self.instructions.push(Instruction::Turn(turn));
                    distance = 0;
                }
            }
        }
        if distance > 0 {
            self.instructions.push(Instruction::Distance(distance));
        }

        if cfg!(test) {
            for row in 0..self.height {
                for cell in 0..self.width {
                    print!(
                        "{}",
                        match self.board.get(&Point(row, cell)) {
                            Some(Cell::Open) => '.',
                            Some(Cell::Solid) => '#',
                            None => ' ',
                        }
                    );
                }
                println!();
            }
            println!("{:?}", self.instructions);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut pos = Point(0, 0);
        for c in 0..self.width {
            if self.board.get(&Point(0, c)).is_some() {
                pos.1 = c;
                break;
            }
        }
        let mut dir = Dir::East;
        for instruction in &self.instructions {
            (pos, dir) = self.step(instruction, pos, dir);
        }
        let score = (pos.0 + 1) * 1000
            + (pos.1 + 1) * 4
            + match dir {
                Dir::East => 0,
                Dir::South => 1,
                Dir::West => 2,
                Dir::North => 3,
            };
        output(score)
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl AocDay {
    fn step(
        &self,
        instruction: &Instruction,
        mut pos: Point<i64>,
        heading: Dir,
    ) -> (Point<i64>, Dir) {
        match instruction {
            Instruction::Turn(turn) => match (heading, turn) {
                (Dir::North, Turn::Right) | (Dir::South, Turn::Left) => (pos, Dir::East),
                (Dir::North, Turn::Left) | (Dir::South, Turn::Right) => (pos, Dir::West),
                (Dir::East, Turn::Right) | (Dir::West, Turn::Left) => (pos, Dir::South),
                (Dir::East, Turn::Left) | (Dir::West, Turn::Right) => (pos, Dir::North),
            },
            Instruction::Distance(distance) => {
                for _ in 0..*distance {
                    let mut count = 0;
                    let mut next_pos = pos;
                    loop {
                        next_pos += heading.value();
                        next_pos.0 = next_pos.0.rem_euclid(self.height); // Wrap the height
                        next_pos.1 = next_pos.1.rem_euclid(self.width); // Wrap the width
                        if self.board.get(&next_pos).is_some() {
                            // Walk until we're back in the valid area
                            break;
                        }
                        count += 1;
                        if count > self.width.max(self.height) {
                            break;
                        };
                    }
                    if let Some(Cell::Solid) = self.board.get(&next_pos) {
                        // Stop walking at solid wall.
                        break;
                    };
                    pos = next_pos;
                }

                (pos, heading)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Open,
    Solid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Distance(i64),
    Turn(Turn),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Turn {
    Right,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        assert_eq!(Instruction::Distance(5), *day.instructions.last().unwrap());
        let expected = 6032;
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
