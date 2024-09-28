use std::str::FromStr;

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    report: Vec<BinaryNumber>,
    rows: usize,
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
        self.report = aoc::read_lines(&self.input)
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();
        self.rows = self.report.len();
    }

    fn part1(&mut self) -> Vec<String> {
        let bit_totals = bit_totals(&self.report.iter().collect::<Vec<_>>());
        let gamma = BinaryNumber::from_bit_totals(&bit_totals, self.report.len());
        let epsilon = gamma.flip();
        output(gamma.value() * epsilon.value())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut o2 = self.report.iter().collect::<Vec<_>>();
        let mut co2 = self.report.iter().collect::<Vec<_>>();
        let mut idx = 0;
        while o2.len() != 1 {
            let bit_totals = BinaryNumber::from_bit_totals(&bit_totals(o2.as_slice()), o2.len());
            o2 = filter(o2.as_slice(), &bit_totals, idx);
            idx += 1;
        }
        idx = 0;
        while co2.len() != 1 {
            let bit_totals =
                BinaryNumber::from_bit_totals(&bit_totals(co2.as_slice()), co2.len()).flip();
            co2 = filter(co2.as_slice(), &bit_totals, idx);
            idx += 1;
        }
        output(o2[0].value() * co2[0].value())
    }
}

fn bit_totals(bits: &[&BinaryNumber]) -> Vec<usize> {
    let length = bits[0].values.len();
    let result = bits.iter().fold(vec![0; length], |mut acc, x| {
        for (idx, val) in x.values.iter().enumerate() {
            acc[idx] += *val as usize;
        }
        acc
    });
    result
}

fn filter<'a>(
    bits: &[&'a BinaryNumber],
    filter_value: &BinaryNumber,
    idx: usize,
) -> Vec<&'a BinaryNumber> {
    bits.iter()
        .filter_map(|val| {
            if val.compare(filter_value, idx) {
                Some(*val)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, Default, Clone)]
struct BinaryNumber {
    values: Vec<u8>,
}

impl BinaryNumber {
    fn value(&self) -> u64 {
        let string = self
            .values
            .iter()
            .map(|b| (*b + 48) as char)
            .collect::<String>();
        u64::from_str_radix(&string, 2).expect("Bad String")
    }

    fn flip(&self) -> Self {
        Self {
            values: self
                .values
                .iter()
                .map(|v| if *v == 1 { 0 } else { 1 })
                .collect(),
        }
    }

    fn from_bit_totals(bit_totals: &[usize], size: usize) -> Self {
        let values = bit_totals.iter().map(|b| (*b >= size - b) as u8).collect();
        Self { values }
    }

    fn compare(&self, other: &Self, bit: usize) -> bool {
        self.values[bit] == other.values[bit]
    }
}

impl FromStr for BinaryNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s.chars().map(|c| c as u8 - 48).collect(),
        })
    }
}
