use std::str::FromStr;

fn main() {
    println!("---- 2025: 06 ----");
    let input = puzlib::read_lines("aoc2025/inputs/day06.txt");
    println!("Parsing");
    let model = parse(input);
    println!("Part 1: {}", part1(&model));
    println!("Part 2: {}", part2(&model));
}

fn parse(input: Vec<String>) -> Vec<Operation> {
    let mut ops = input[0]
        .split_whitespace()
        .map(|value| Operation {
            numbers: vec![value.parse().unwrap()],
            operator: Operator::None,
        })
        .collect::<Vec<_>>();
    for line in input.into_iter().skip(1) {
        for (idx, val) in line.split_whitespace().enumerate() {
            match val.parse::<i64>() {
                Ok(n) => ops[idx].numbers.push(n),
                Err(_) => ops[idx].operator = val.parse().unwrap(),
            }
        }
    }
    ops
}

fn part1(model: &[Operation]) -> i64 {
    model.iter().map(Operation::process).sum()
}

fn part2(_model: &[Operation]) -> String {
    "Unsolved".into()
}

struct Operation {
    numbers: Vec<i64>,
    operator: Operator,
}

impl Operation {
    fn process(&self) -> i64 {
        match self.operator {
            Operator::Add => self.numbers.iter().fold(0, |acc, n| acc + *n),
            Operator::Mul => self.numbers.iter().fold(1, |acc, n| acc * *n),
            Operator::None => panic!("None operator found."),
        }
    }
}

enum Operator {
    Add,
    Mul,
    None,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(format!("Unknown operator {s}")),
        }
    }
}
