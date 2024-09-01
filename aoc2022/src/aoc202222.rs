use std::collections::HashMap;

use aoc::{
    runner::{output, Runner},
    Dir, Point,
};

const EDGE: i64 = if cfg!(test) { 4 } else { 50 }; // Edge size of the cube face

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    pub(crate) board: HashMap<Point<i64>, Cell>,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) width: i64,
    pub(crate) height: i64,
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
                        ' ' => None,
                        _ => panic!("Unknown character {ch}"),
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
            if self.board.contains_key(&Point(0, c)) {
                pos.1 = c;
                break;
            }
        }
        let mut dir = Dir::East;
        for instruction in &self.instructions {
            (pos, dir) = self.flat_step(instruction, pos, dir);
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
        let mut pos = Point(0, 0);
        for c in 0..self.width {
            if self.board.contains_key(&Point(0, c)) {
                pos.1 = c;
                break;
            }
        }
        let mut dir = Dir::East;
        for instruction in &self.instructions {
            (pos, dir) = self.cube_step(instruction, pos, dir);
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
}

impl AocDay {
    fn flat_step(
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
                    let mut next_pos = pos;
                    loop {
                        next_pos += heading.value();
                        next_pos.0 = next_pos.0.rem_euclid(self.height); // Wrap the height
                        next_pos.1 = next_pos.1.rem_euclid(self.width); // Wrap the width
                        if self.board.contains_key(&next_pos) {
                            // Walk until we're back in the valid area
                            break;
                        }
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

    fn cube_step(
        &self,
        instruction: &Instruction,
        mut pos: Point<i64>,
        mut heading: Dir,
    ) -> (Point<i64>, Dir) {
        // This won't be generic to any input. The input layout is for this specific wrapping.
        // .## -> A, B
        // .#. -> C
        // ##. -> D, E
        // #.. -> F
        // Matching edges:
        // A up F left
        // A left D left
        // B up F down
        // B right E right
        // B down C right
        // C left D up
        // E down F right
        // All tiles declared to be 50x50.

        // Test input is unwrapped to
        // ..#.
        // ###.
        // ..##
        // and won't be tested.

        match instruction {
            // Turns are no different from other version.
            Instruction::Turn(turn) => match (heading, turn) {
                (Dir::North, Turn::Right) | (Dir::South, Turn::Left) => (pos, Dir::East),
                (Dir::North, Turn::Left) | (Dir::South, Turn::Right) => (pos, Dir::West),
                (Dir::East, Turn::Right) | (Dir::West, Turn::Left) => (pos, Dir::South),
                (Dir::East, Turn::Left) | (Dir::West, Turn::Right) => (pos, Dir::North),
            },
            Instruction::Distance(distance) => {
                for _ in 0..*distance {
                    let mut next_pos = pos + heading.value();
                    let mut temp_heading = heading;
                    if !self.board.contains_key(&next_pos) {
                        // Stepped off the edge
                        if heading == Dir::North {
                            // Faces A, B, D
                            if (0..50).contains(&next_pos.1) {
                                // Face D top -> C left (C0->R50, C49->R99)
                                next_pos = Point(pos.1 + EDGE, EDGE);
                                temp_heading = Dir::East;
                            } else if (50..100).contains(&next_pos.1) {
                                // Face A top -> F left (C50->R150, C99->R149)
                                next_pos = Point(pos.1 + 2 * EDGE, 0);
                                temp_heading = Dir::East;
                            } else {
                                // Face B top -> F bottom (C100->C0, C149->C49)
                                next_pos = Point(4 * EDGE - 1, pos.1 - 2 * EDGE);
                            }
                        } else if heading == Dir::East {
                            // Faces B, C, E, F
                            if (0..50).contains(&next_pos.0) {
                                // Face B right -> E right (R0->R149, R49->R100)
                                // NewRow = 3 * EDGE - 1 - OldRow
                                next_pos = Point(3 * EDGE - pos.0 - 1, 2 * EDGE - 1);
                                temp_heading = Dir::West;
                            } else if (50..100).contains(&next_pos.0) {
                                // Face C right -> B bottom (R50->C100, R99->C149)
                                next_pos = Point(EDGE - 1, pos.0 + EDGE);
                                temp_heading = Dir::North;
                            } else if (100..150).contains(&next_pos.0) {
                                // Face E right -> B right (R149->R100, R100->R149)
                                // NewRow = 3 * EDGE - 1 - OldRow
                                next_pos = Point(3 * EDGE - pos.0 - 1, 3 * EDGE - 1);
                                temp_heading = Dir::West;
                            } else {
                                // Face F right -> E bottom (R150->C50, R199->C99)
                                next_pos = Point(3 * EDGE - 1, pos.0 - 2 * EDGE);
                                temp_heading = Dir::North;
                            }
                        } else if heading == Dir::South {
                            // Faces F, E, B
                            if (0..50).contains(&next_pos.1) {
                                // Face F bottom -> B top (C0->C100, C49->C149)
                                next_pos = Point(0, pos.1 + 2 * EDGE);
                            } else if (50..100).contains(&next_pos.1) {
                                // Face E bottom -> F right (C50->R150, C99-R199)
                                next_pos = Point(pos.1 + 2 * EDGE, EDGE - 1);
                                temp_heading = Dir::West;
                            } else {
                                // Face B bottom -> C right (C100->R50, C149->R99)
                                next_pos = Point(pos.1 - EDGE, 2 * EDGE - 1);
                                temp_heading = Dir::West
                            }
                        } else {
                            // heading = Dir::West
                            // Faces A, C, D, F
                            if (0..50).contains(&next_pos.0) {
                                // Face A left -> D left (R0->R149, R49->R100)
                                // NewRow = 3 * EDGE - 1 - OldRow
                                next_pos = Point(3 * EDGE - 1 - pos.0, 0);
                                temp_heading = Dir::East;
                            } else if (50..100).contains(&next_pos.0) {
                                // Face C left -> D top (R50->C0, R99->C49)
                                next_pos = Point(2 * EDGE, pos.0 - EDGE);
                                temp_heading = Dir::South;
                            } else if (100..150).contains(&next_pos.0) {
                                // Face D left -> A left (R100->R49, R149->R0)
                                // NewRow = 3 * EDGE - 1 - OldRow
                                next_pos = Point(3 * EDGE - 1 - pos.0, EDGE);
                                temp_heading = Dir::East;
                            } else {
                                // Face F left -> A top (R150->C50, R199->C99)
                                next_pos = Point(0, pos.0 - 2 * EDGE);
                                temp_heading = Dir::South;
                            }
                        };
                    }
                    if let Some(Cell::Solid) = self.board.get(&next_pos) {
                        // Hit a wall, exit early.
                        break;
                    }
                    if !self.board.contains_key(&next_pos) {
                        // Since I'm not writing the conditions for the test input,
                        // verify we're always on the map.
                        panic!("Ran off the edge at {pos:?} going {heading:?}, mapped to {next_pos:?} going {temp_heading:?}");
                    };
                    pos = next_pos;
                    heading = temp_heading;
                }

                (pos, heading)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Cell {
    Open,
    Solid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Instruction {
    Distance(i64),
    Turn(Turn),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Turn {
    Right,
    Left,
}
