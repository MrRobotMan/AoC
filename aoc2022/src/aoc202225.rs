use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    burners: Vec<String>,
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
        (2022, 25)
    }

    fn parse(&mut self) {
        self.burners = aoc::read_lines(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self.burners.iter().map(decrypt).sum::<i64>();
        output(encrypt(total))
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

fn encrypt(mut value: i64) -> String {
    let chars = ['0', '1', '2', '=', '-'];
    let mut res = String::new();
    while value > 0 {
        res.push(chars[value as usize % 5]);
        value = (value + 2) / 5;
    }
    if res.is_empty() {
        res.push('0');
    }
    res.chars().rev().collect()
}

fn decrypt<T: AsRef<str>>(value: T) -> i64 {
    value
        .as_ref()
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (pow, val)| {
            acc + (match val {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!("Unknown val"),
            }) * 5_i64.pow(pow as u32)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let total = 4890;
        assert_eq!(total, day.burners.iter().map(decrypt).sum::<i64>());
        let expected = "2=-1=0";
        let actual = &day.part1()[0];
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
