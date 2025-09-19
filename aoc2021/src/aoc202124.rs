use std::str::FromStr;

use aoc::{
    read_lines,
    runner::{output, Runner},
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
        'attempt: for model in (11_111_111_111_111_i64..=99_999_999_999_999).rev() {
            let mut inp;
            let mut rem = model;
            let mut alu = Alu::default();
            let mut loc = 13;
            for instruction in &self.instructions {
                match instruction {
                    Instruction::Inp(reg) => {
                        inp = rem / 10_i64.pow(loc);
                        if inp == 0 {
                            continue 'attempt;
                        }
                        loc -= 1;
                        rem %= 10_i64.pow(loc);
                        alu.inp(*reg, inp);
                    }
                    Instruction::Add(reg, value) => alu.add(*reg, *value),
                    Instruction::Mul(reg, value) => alu.mul(*reg, *value),
                    Instruction::Div(reg, value) => alu.div(*reg, *value),
                    Instruction::Mod(reg, value) => alu.modulo(*reg, *value),
                    Instruction::Eql(reg, value) => alu.equal(*reg, *value),
                }
            }
            if alu.registers[3] == 0 {
                return output(model);
            }
        }
        output("Unsolved")
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
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
            "inp" => Self::Inp(reg),
            "add" => Self::Add(reg, values[2].parse().unwrap()),
            "mul" => Self::Mul(reg, values[2].parse().unwrap()),
            "div" => Self::Div(reg, values[2].parse().unwrap()),
            "mod" => Self::Mod(reg, values[2].parse().unwrap()),
            "eql" => Self::Eql(reg, values[2].parse().unwrap()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Reg(usize),
    Num(i64),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Self::Reg(0),
            "x" => Self::Reg(1),
            "y" => Self::Reg(2),
            "z" => Self::Reg(3),
            _ => Self::Num(s.parse().unwrap()),
        })
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Alu {
    registers: [i64; 4],
}

impl Alu {
    fn inp(&mut self, reg: usize, value: i64) {
        self.registers[reg] = value;
    }

    fn add(&mut self, reg: usize, value: Value) {
        self.registers[reg] += self.value(value);
    }

    fn mul(&mut self, reg: usize, value: Value) {
        self.registers[reg] *= self.value(value);
    }

    fn div(&mut self, reg: usize, value: Value) {
        if self.value(value) == 0 {
            panic!("Div by 0");
        }
        self.registers[reg] /= self.value(value);
    }

    fn modulo(&mut self, reg: usize, value: Value) {
        if self.registers[reg] < 0 || self.value(value) <= 0 {
            panic!("Bad modulo");
        }
        self.registers[reg] %= self.value(value);
    }

    fn equal(&mut self, reg: usize, value: Value) {
        self.registers[reg] = (self.registers[reg] == self.value(value)) as i64;
    }

    fn value(&self, value: Value) -> i64 {
        match value {
            Value::Reg(b) => self.registers[b],
            Value::Num(n) => n,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_storage() -> Result<(), String> {
        let mut alu = Alu::default();
        for instruction in "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"
            .split('\n')
            .map(|st| st.parse::<Instruction>().unwrap())
        {
            match instruction {
                Instruction::Inp(reg) => alu.inp(reg, 13),
                Instruction::Add(reg, value) => alu.add(reg, value),
                Instruction::Mul(reg, value) => alu.mul(reg, value),
                Instruction::Div(reg, value) => alu.div(reg, value),
                Instruction::Mod(reg, value) => alu.modulo(reg, value),
                Instruction::Eql(reg, value) => alu.equal(reg, value),
            }
        }
        let expected = [1, 1, 0, 1];
        let actual = alu.registers;
        assert_eq!(expected, actual);
        Ok(())
    }
}
