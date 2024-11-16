use aoc::{
    read_line_record,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    ages: Vec<u64>,
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
        (2021, 6)
    }

    fn parse(&mut self) {
        self.ages = read_line_record(&self.input);
    }

    fn part1(&mut self) -> String {
        output(self.simulate(80))
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

impl AocDay {
    fn simulate(&mut self, days: u64) -> usize {
        for _ in 0..days {
            self.step_day();
        }
        self.ages.len()
    }

    fn step_day(&mut self) {
        let mut born = 0;
        for fish in self.ages.iter_mut() {
            if *fish == 0 {
                born += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        self.ages.extend_from_slice(&vec![8; born]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            ages: vec![3, 4, 3, 1, 2],
            ..Default::default()
        };
        assert_eq!(5934, day.simulate(80));
    }
}
