use std::{fmt::Display, ops::Deref, str::Chars};

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    numbers: Vec<Number>,
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
        (2021, 18)
    }

    fn parse(&mut self) {
        self.numbers = read_lines(&self.input).iter().map(Number::new).collect();
    }

    fn part1(&mut self) -> String {
        output(
            self.numbers
                .iter()
                .skip(1)
                .fold(self.numbers[0].clone(), |acc, n| acc + n)
                .magnitude(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Number {
    Value(u8),
    Pair(Box<[Number; 2]>),
}

impl Default for Number {
    fn default() -> Self {
        Self::Value(0)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(pair) => {
                let [left, right] = &pair.deref();
                write!(f, "[")?;
                write!(f, "{left}")?;
                write!(f, ",")?;
                write!(f, "{right}")?;
                write!(f, "]")
            }
            Self::Value(value) => write!(f, "{value}"),
        }
    }
}

impl Number {
    fn new<T: AsRef<str>>(value: T) -> Self {
        let mut iter = value.as_ref().chars();
        let n = Self::process(&mut iter);
        println!("{n}");
        n
    }

    fn process(iter: &mut Chars) -> Self {
        let left = match iter.next() {
            Some('[') => Self::process(iter),
            None | Some(',') | Some(']') => unreachable!(),
            Some(v) => Self::Value(v as u8 - b'0'),
        };
        // Skip the comma. Don't skip closing brace if the right is just a number.
        if iter.next().is_none() {
            return left;
        };
        let right = match iter.next() {
            Some('[') => Self::process(iter),
            None | Some(',') | Some(']') => unreachable!(),
            Some(v) => Self::Value(v as u8 - b'0'),
        };
        iter.next(); // Skip the closing bracket
        Self::Pair(Box::new([left, right]))
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Value(v) => *v as usize,
            Self::Pair(pair) => 3 * pair[0].magnitude() + 2 * pair[1].magnitude(),
        }
    }

    fn reduce(&mut self) {
        let mut reduced = false;
        while !reduced {
            reduced = self.explode() || self.split();
        }
    }

    fn explode(&mut self) -> bool {
        true
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Value(v) if *v >= 10 => {
                let left = *v / 2;
                let right = *v - left;
                *self = Self::Pair(Box::new([Self::Value(left), Self::Value(right)]));
                true
            }
            Self::Value(_) => false,
            Self::Pair(p) => p[0].split() || p[1].split(),
        }
    }
}

impl std::ops::Add<&Number> for Number {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut number = Self::Pair(Box::new([self, rhs.clone()]));
        number.reduce();
        number
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_number() {
        let actual = Number::new("[[1,2],3]");
        let expected = Number::Pair(Box::new([
            Number::Pair(Box::new([Number::Value(1), Number::Value(2)])),
            Number::Value(3),
        ]));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_basic_number2() {
        let actual = Number::new("[[1,9],[8,5]]");
        let expected = Number::Pair(Box::new([
            Number::Pair(Box::new([Number::Value(1), Number::Value(9)])),
            Number::Pair(Box::new([Number::Value(8), Number::Value(5)])),
        ]));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_magnitude() {
        let expected = 143;
        let actual = Number::new("[[1,2],[[3,4],5]]").magnitude();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_explode() {
        let expected = Number::new("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        let mut actual = Number::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        actual.explode();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_split() {
        use Number::*;
        let expected = Pair(Box::new([
            Pair(Box::new([
                Pair(Box::new([Pair(Box::new([Value(0), Value(7)])), Value(4)])),
                Pair(Box::new([
                    Pair(Box::new([Value(7), Value(8)])),
                    Pair(Box::new([Value(0), Value(13)])),
                ])),
            ])),
            Pair(Box::new([Value(1), Value(1)])),
        ]));
        assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", &format!("{expected}"));
        let mut actual = Pair(Box::new([
            Pair(Box::new([
                Pair(Box::new([Pair(Box::new([Value(0), Value(7)])), Value(4)])),
                Pair(Box::new([Value(15), Pair(Box::new([Value(0), Value(13)]))])),
            ])),
            Pair(Box::new([Value(1), Value(1)])),
        ]));
        assert_eq!("[[[[0,7],4],[15,[0,13]]],[1,1]]", &format!("{actual}"));
        actual.split();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_example() {
        let numbers = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            .lines()
            .map(|line| Number::new(line.trim()))
            .collect::<Vec<_>>();
        let expected = 4140;
        let actual = numbers
            .iter()
            .skip(1)
            .fold(numbers[0].clone(), |acc, n| acc + n)
            .magnitude();
        assert_eq!(expected, actual);
    }
}
