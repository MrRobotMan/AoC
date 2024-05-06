use aoc::{
    read_lines,
    runner::{output, Runner},
    Point,
};

#[derive(Default)]
pub struct AocDay {
    input: String,
    position: Point<usize>,
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

    fn part1(&mut self) -> Vec<String> {
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Forward(x) => self.position.0 += x,
                Instruction::Down(x) => self.position.1 += x,
                Instruction::Up(x) => self.position.1 = self.position.1.saturating_sub(*x),
            }
        }
        output(self.position.0 * self.position.1)
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
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
