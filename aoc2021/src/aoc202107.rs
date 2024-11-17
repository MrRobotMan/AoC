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
        self.crabs.sort();
    }

    fn part1(&mut self) -> String {
        let len = self.crabs.len();
        let steps = if len % 2 == 1 {
            self.step_total(self.crabs[len / 2 + 1]) // median.
        } else {
            self.step_total(self.crabs[len / 2]) // Left of midpoint.
                .min(self.step_total(self.crabs[len / 2 + 1])) // Right of midpoint.
        };

        output(steps)
    }

    fn part2(&mut self) -> String {
        let mut steps = usize::MAX;
        for position in 0..self.crabs.len() {
            steps = steps.min(self.weighted_step_total(position))
        }
        output(steps)
    }
}

impl AocDay {
    fn step_total(&self, idx: usize) -> usize {
        self.crabs
            .iter()
            .map(|v| if *v > idx { *v - idx } else { idx - *v })
            .sum()
    }

    fn weighted_step_total(&self, idx: usize) -> usize {
        self.crabs
            .iter()
            .map(|v| {
                if *v > idx {
                    triangular(*v - idx)
                } else {
                    triangular(idx - *v)
                }
            })
            .sum()
    }
}

fn triangular(number: usize) -> usize {
    (number * (number + 1)) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_triangular() {
        assert_eq!(10, triangular(4));
        assert_eq!(45, triangular(9));
    }

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            crabs: vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14],
            ..Default::default()
        };

        assert_eq!("37", &day.part1());
    }

    #[test]
    fn test_example2() {
        let mut day = AocDay {
            crabs: vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14],
            ..Default::default()
        };

        assert_eq!("168", &day.part2());
    }
}
