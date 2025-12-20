use std::str::FromStr;

use aoc::{
    read_lines,
    runner::{Runner, output},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
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
        (2021, 24)
    }

    fn parse(&mut self) {
        self.instructions = read_lines(&self.input)
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> String {
        let divs = self.simplify();
        output(self.find_models(divs, Find::Max))
    }

    fn part2(&mut self) -> String {
        let divs = self.simplify();
        output(self.find_models(divs, Find::Min))
    }
}

impl AocDay {
    fn simplify(&self) -> Vec<Addend> {
        let mut res = vec![];
        for input_index in (0..self.instructions.len()).step_by(18) {
            match self.instructions[input_index + 4] {
                Instruction::Div(3, Value::Num(1)) => {
                    if let Instruction::Add(2, Value::Num(addend)) =
                        self.instructions[input_index + 15]
                    {
                        res.push(Addend::Div1(addend));
                    }
                }
                Instruction::Div(3, Value::Num(26)) => {
                    if let Instruction::Add(1, Value::Num(addend)) =
                        self.instructions[input_index + 5]
                    {
                        res.push(Addend::Div26(addend));
                    }
                }
                _ => (),
            }
        }

        res
    }
    fn find_models(&self, divs: Vec<Addend>, find: Find) -> i64 {
        let mut model = [0; 14];
        let mut stack = vec![];
        for (dig, addend) in divs.iter().enumerate() {
            match addend {
                Addend::Div1(value) => stack.push((dig, *value)),
                Addend::Div26(value) => {
                    if let Some((idx, v)) = stack.pop() {
                        let diff = v + value;
                        match find {
                            Find::Max => {
                                model[idx] = 9.min(9 - diff);
                                model[dig] = 9.min(9 + diff);
                            }
                            Find::Min => {
                                model[idx] = 1.max(1 - diff);
                                model[dig] = 1.max(1 + diff);
                            }
                        }
                    };
                }
            }
        }
        model.iter().fold(0, |acc, v| acc * 10 + v)
    }
}

enum Find {
    Min,
    Max,
}

#[derive(Copy, Clone, Debug)]
enum Addend {
    Div1(i64),
    Div26(i64),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Add(usize, Value),
    Div(usize, Value),
    Unimportant,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split_ascii_whitespace().collect::<Vec<_>>();
        let reg = match values[1] {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => unreachable!(),
        };
        Ok(match values[0] {
            "add" => Self::Add(reg, values[2].parse().unwrap()),
            "div" => Self::Div(reg, values[2].parse().unwrap()),
            _ => Self::Unimportant,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Reg,
    Num(i64),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" | "x" | "y" | "z" => Self::Reg,
            _ => Self::Num(s.parse().unwrap()),
        })
    }
}
