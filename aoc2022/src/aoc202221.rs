use std::collections::HashMap;

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    monkeys: HashMap<String, Monkey>,
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
        let (left, right) = if let Monkey::Operation(left, _, right) = &self.monkeys["root"] {
            (left, right)
        } else {
            panic!("Root is constant");
        };
        let mut path = self.find_human("root").unwrap();
        path.reverse();
        let total = if left == &path[1] {
            self.monkeys[right].process(&self.monkeys)
        } else {
            self.monkeys[left].process(&self.monkeys)
        };
        output(self.adjust_total(&path, 1, total))
    }
}

impl AocDay {
    fn find_human(&self, node: &str) -> Option<Vec<String>> {
        if node == "humn" {
            return Some(vec!["humn".into()]);
        }

        if let Monkey::Operation(ref left, _, ref right) = self.monkeys[node] {
            if let Some(mut vec) = self.find_human(left) {
                vec.push(node.into());
                return Some(vec);
            }
            if let Some(mut vec) = self.find_human(right) {
                vec.push(node.into());
                return Some(vec);
            }
        }

        None
    }

    fn adjust_total(&self, path: &[String], idx: usize, total: i64) -> i64 {
        match self.monkeys[&path[idx]] {
            Monkey::Constant(_) => total,
            Monkey::Operation(ref lhs, ope, ref rhs) => {
                let left = self.monkeys[lhs].process(&self.monkeys);
                let right = self.monkeys[rhs].process(&self.monkeys);
                let adjustment = if lhs == &path[idx + 1] {
                    // A = x + y => x = A - y
                    // A = x - y => x = A + y
                    // A = x * y => x = A / y
                    // A = x / y => x = A * y
                    match ope.0 {
                        '+' => total - right,
                        '-' => total + right,
                        '*' => total / right,
                        '/' => total * right,
                        _ => unreachable!(),
                    }
                } else {
                    match ope.0 {
                        // A = x + y => y = A - x
                        // A = x - y => y = x - A
                        // A = x * y => y = A / x
                        // A = x / y => y = x / A
                        '+' => total - left,
                        '-' => left - total,
                        '*' => total / left,
                        '/' => left / total,
                        _ => unreachable!(),
                    }
                };
                self.adjust_total(path, idx + 1, adjustment)
            }
        }
    }
}

#[derive(Clone)]
enum Monkey {
    Constant(i64),
    Operation(String, (char, fn(i64, i64) -> i64), String),
}

impl Monkey {
    fn process(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Self::Constant(c) => *c,
            Self::Operation(lhs, ope, rhs) => {
                ope.1(monkeys[lhs].process(monkeys), monkeys[rhs].process(monkeys))
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
                {
                    let c = values[1].chars().next().unwrap();
                    (
                        c,
                        match c {
                            '+' => |lhs, rhs| lhs + rhs,
                            '-' => |lhs, rhs| lhs - rhs,
                            '*' => |lhs, rhs| lhs * rhs,
                            '/' => |lhs, rhs| lhs / rhs,
                            _ => unreachable!("Unknown operation"),
                        },
                    )
                },
                values[2].into(),
            ),
            _ => unreachable!("Value has the wrong number of parts"),
        }
    }
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Monkey::Constant(c) => write!(f, "{c}"),
            Monkey::Operation(lhs, (ope, _), rhs) => write!(f, "{lhs} {ope} {rhs}"),
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
        let expected = 301;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
