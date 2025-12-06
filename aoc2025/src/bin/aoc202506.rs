fn main() {
    println!("---- 2025: 06 ----");
    let input = puzlib::contents("aoc2025/inputs/day06.txt")
        .lines()
        .map(str::to_string)
        .collect::<Vec<_>>();
    println!("Parsing");
    let [p1, p2] = parse(input);
    println!("Part 1: {}", part1(&p1));
    println!("Part 2: {}", part2(&p2));
}

fn parse(mut input: Vec<String>) -> [Vec<Operation>; 2] {
    let mut part1 = vec![];
    let mut part2 = vec![];
    let mut breaks = vec![];
    // Find the start indicies of the numbers. Operators are aligned with the left of each start.
    let mut start = 0;
    for (idx, ch) in input.pop().unwrap().chars().enumerate() {
        if let Ok(op) = ch.try_into() {
            if idx != 0 {
                breaks.push((start, idx - 1));
                start = idx;
            }
            let v = Operation {
                numbers: vec![],
                operator: op,
            };
            part1.push(v.clone());
            part2.push(v);
        }
    }
    breaks.push((start, input[0].len()));
    for line in input.iter() {
        for (idx, (start, end)) in breaks.iter().enumerate() {
            let n = &line[*start..*end];
            part1[idx].numbers.push(n.trim().parse().unwrap());

            for (step, ch) in n.chars().enumerate() {
                if part2[idx].numbers.len() <= step || part2[idx].numbers.is_empty() {
                    part2[idx].numbers.push(0);
                }
                if ch.is_ascii_digit() {
                    part2[idx].numbers[step] =
                        part2[idx].numbers[step] * 10 + (ch as u8 - b'0') as i64;
                }
            }
        }
    }
    [part1, part2]
}

fn part1(model: &[Operation]) -> i64 {
    model.iter().map(Operation::process).sum()
}

fn part2(model: &[Operation]) -> i64 {
    model.iter().map(Operation::process).sum()
}

#[derive(Debug, PartialEq, Clone)]
struct Operation {
    numbers: Vec<i64>,
    operator: Operator,
}

impl Operation {
    fn process(&self) -> i64 {
        match self.operator {
            Operator::Add => self.numbers.iter().fold(0, |acc, n| acc + *n),
            Operator::Mul => self.numbers.iter().fold(1, |acc, n| acc * *n),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operator {
    Add,
    Mul,
}

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing_part2() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        let expected1 = vec![
            Operation {
                numbers: vec![123, 45, 6],
                operator: Operator::Mul,
            },
            Operation {
                numbers: vec![328, 64, 98],
                operator: Operator::Add,
            },
            Operation {
                numbers: vec![51, 387, 215],
                operator: Operator::Mul,
            },
            Operation {
                numbers: vec![64, 23, 314],
                operator: Operator::Add,
            },
        ];
        let expected2 = vec![
            Operation {
                numbers: vec![1, 24, 356],
                operator: Operator::Mul,
            },
            Operation {
                numbers: vec![369, 248, 8],
                operator: Operator::Add,
            },
            Operation {
                numbers: vec![32, 581, 175],
                operator: Operator::Mul,
            },
            Operation {
                numbers: vec![623, 431, 4],
                operator: Operator::Add,
            },
        ];
        let [actual1, actual2] = parse(input);
        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
    }
}
