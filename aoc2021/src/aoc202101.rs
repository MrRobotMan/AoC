use aoc::{
    read_numbers,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    input: String,
    measurements: Vec<(usize, Delta)>,
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
        let measurements = read_numbers(&self.input);
        self.measurements
            .push((measurements[0], Delta::NotApplicable));
        for (idx, depth) in measurements.iter().skip(1).enumerate() {
            self.measurements.push((
                *depth,
                match depth.cmp(&self.measurements[idx].0) {
                    std::cmp::Ordering::Less => Delta::Decrease,
                    std::cmp::Ordering::Equal => Delta::NoChange,
                    std::cmp::Ordering::Greater => Delta::Increase,
                },
            ))
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.measurements.iter().fold(0, |acc, (_, delta)| {
            acc + if matches!(delta, Delta::Increase) {
                1
            } else {
                0
            }
        }))
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug)]
enum Delta {
    NoChange,
    Increase,
    Decrease,
    NotApplicable,
}
