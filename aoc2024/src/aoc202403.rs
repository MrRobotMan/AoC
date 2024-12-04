use aoc::{
    contents,
    runner::{output, Runner},
};
use regex::Regex;

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    data: String,
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
        (2024, 3)
    }

    fn parse(&mut self) {
        self.data = contents(&self.input);
    }

    fn part1(&mut self) -> String {
        let re = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
        output(
            re.captures_iter(&self.data)
                .map(|c| mult(&c[1], &c[2]))
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> String {
        let mut enabled = true;
        let re = Regex::new(r#"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"#).unwrap();
        let mut total = 0;
        for cap in re.captures_iter(&self.data) {
            match &cap[0] {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        total += mult(&cap[1], &cap[2]);
                    }
                }
            }
        }
        output(total)
    }
}

fn mult(left: &str, right: &str) -> i64 {
    left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            input: String::new(),
            data: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)))".into(),
        };
        let expected = "161";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let mut day = AocDay {
            input: String::new(),
            data: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)))"
                .into(),
        };
        let expexted = "48";
        let actual = day.part2();
        assert_eq!(expexted, actual);
    }
}
