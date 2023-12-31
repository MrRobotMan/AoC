use std::collections::HashMap;

use aoc::runner::{output, run_solution, Runner};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day21.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    monkeys: HashMap<String, Monkey>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 21)
    }

    fn parse(&mut self) {
        self.monkeys = aoc::read_lines(&self.input)
            .iter()
            .map(|line| {
                let (id, steps) = line.split_once(": ").unwrap();
                (id.into(), steps.into())
            })
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.monkeys["root"].process(&self.monkeys))
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

enum Monkey {
    Constant(i64),
    Operation(String, Box<dyn Fn(i64, i64) -> i64>, String),
}

impl Monkey {
    fn process(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Self::Constant(c) => *c,
            Self::Operation(lhs, ope, rhs) => {
                ope(monkeys[lhs].process(monkeys), monkeys[rhs].process(monkeys))
            }
        }
    }
}

impl<T: AsRef<str>> From<T> for Monkey {
    fn from(value: T) -> Self {
        let values = value.as_ref().split_ascii_whitespace().collect::<Vec<_>>();
        match values.len() {
            1 => Self::Constant(values[0].parse().unwrap()),
            3 => Self::Operation(
                values[0].into(),
                Box::new(match values[1] {
                    "+" => |lhs, rhs| lhs + rhs,
                    "-" => |lhs, rhs| lhs - rhs,
                    "*" => |lhs, rhs| lhs * rhs,
                    "/" => |lhs, rhs| lhs / rhs,
                    _ => unreachable!("Unknown operation"),
                }),
                values[2].into(),
            ),
            _ => unreachable!("Value has the wrong number of parts"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 152;
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
