use std::collections::HashSet;

use aoc::runner::{output, run_solution, Runner};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day06.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    signal: Vec<char>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 6)
    }

    fn parse(&mut self) {
        self.signal = aoc::read_line(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.process_signal(4))
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.process_signal(14))
    }
}

impl AocDay {
    fn process_signal(&self, signal_len: usize) -> usize {
        for (idx, chunk) in self.signal.windows(signal_len).enumerate() {
            if HashSet::<&char>::from_iter(chunk).len() == signal_len {
                return idx + signal_len;
            }
        }
        self.signal.len()
    }
}
