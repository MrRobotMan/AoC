use aoc::{
    read_line_record,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    crabs: Vec<usize>,
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
        (2021, 7)
    }

    fn parse(&mut self) {
        self.crabs = read_line_record(&self.input);
    }

    fn part1(&mut self) -> String {
        let mut sorted = self.crabs.clone();
        sorted.sort();
        let len = sorted.len();
        let steps = if len % 2 == 1 {
            self.step_total(sorted[len / 2 + 1]) // median.
        } else {
            self.step_total(sorted[len / 2])
                .min(self.step_total(sorted[len / 2 + 1]))
        };

        output(steps)
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

impl AocDay {
    fn step_total(&self, idx: usize) -> usize {
        self.crabs
            .iter()
            .map(|v| if *v > idx { *v - idx } else { idx - *v })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            crabs: vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14],
            ..Default::default()
        };

        assert_eq!("37", &day.part1());
    }
}
