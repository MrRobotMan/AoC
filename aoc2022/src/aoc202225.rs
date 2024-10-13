use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    pub(crate) burners: Vec<String>,
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

    fn part1(&mut self) -> String {
        let total = self.burners.iter().map(decrypt).sum::<i64>();
        output(encrypt(total))
    }

    fn part2(&mut self) -> String {
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

pub(crate) fn decrypt<T: AsRef<str>>(value: T) -> i64 {
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
