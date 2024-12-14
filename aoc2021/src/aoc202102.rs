use aoc::{
    read_lines,
    runner::{output, Runner},
    Vec2D,
};

#[derive(Default)]
pub struct AocDay {
    input: String,
    instructions: Vec<Instruction>,
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
        (2021, 2)
    }

    fn parse(&mut self) {
        // Parse the input
        self.instructions = read_lines(&self.input).iter().map(|l| l.into()).collect();
    }

    fn part1(&mut self) -> String {
        let mut position = Vec2D::<i64>::default();
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Forward(x) => position.0 += x,
                Instruction::Down(x) => position.1 += x,
                Instruction::Up(x) => position.1 -= x,
            }
        }
        output(position.0 * position.1)
    }

    fn part2(&mut self) -> String {
        let mut position = Vec2D::<i64>::default();
        let mut aim = 0;
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Forward(x) => {
                    position.0 += x;
                    position.1 += aim * x;
                }
                Instruction::Down(x) => aim += x,
                Instruction::Up(x) => aim -= x,
            }
        }
        output(position.0 * position.1)
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Forward(0)
    }
}

impl From<&String> for Instruction {
    fn from(value: &String) -> Self {
        let val = value.split_ascii_whitespace().collect::<Vec<_>>();
        match (val[0], val[1]) {
            ("forward", n) => Self::Forward(n.parse().unwrap()),
            ("down", n) => Self::Down(n.parse().unwrap()),
            ("up", n) => Self::Up(n.parse().unwrap()),
            n => unreachable!("Unknown direction {n:?}"),
        }
    }
}
