use aoc::{
    read_numbers,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    input: String,
    measurements: Vec<usize>,
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
        (2021, 1)
    }

    fn parse(&mut self) {
        // Parse the input
        self.measurements = read_numbers(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        output(count_increasing(&self.measurements))
    }

    fn part2(&mut self) -> Vec<String> {
        let sums = self
            .measurements
            .as_slice()
            .windows(3)
            .map(|v| v.iter().sum::<usize>())
            .collect::<Vec<usize>>();
        output(count_increasing(&sums))
    }
}

fn count_increasing(arr: &[usize]) -> usize {
    arr.windows(2).fold(0, |acc, depth| {
        acc + if depth[1].saturating_sub(depth[0]) > 0 {
            1
        } else {
            0
        }
    })
}
