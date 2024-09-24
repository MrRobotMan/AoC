use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    report: Vec<Vec<u8>>,
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
        (2021, 3)
    }

    fn parse(&mut self) {
        self.report = aoc::read_lines(&self.input).iter().map(to_bin).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let rows = self.report.len();
        let bit_totals = &self
            .report
            .iter()
            .fold(vec![0; self.report[0].len()], |mut acc, x| {
                for (idx, val) in x.iter().enumerate() {
                    acc[idx] += *val as usize;
                }
                acc
            });
        let gamma = bit_totals
            .iter()
            .map(|idx| ((*idx > rows / 2) as u8 + 48) as char)
            .collect::<String>();
        let epsilon = gamma
            .chars()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect::<String>();
        output(from_bin(&gamma) * from_bin(&epsilon))
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

/// Convert ASCII value to number. '0' == 48.
fn to_bin(repr: &String) -> Vec<u8> {
    repr.chars().map(|c| c as u8 - 48).collect()
}

/// Convert string to decimal
fn from_bin(value: &str) -> usize {
    usize::from_str_radix(value, 2).expect("Bad string")
}
