use aoc::{
    read_line_record,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    ages: [usize; 9],
    starting: Vec<usize>,
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
        self.starting = read_line_record(&self.input);
        self.reset();
    }

    fn part1(&mut self) -> String {
        output(self.simulate(80))
    }

    fn part2(&mut self) -> String {
        self.reset();
        output(self.simulate(256))
    }
}

impl AocDay {
    fn simulate(&mut self, days: u64) -> usize {
        for _ in 0..days {
            self.step_day();
        }
        self.ages.iter().sum()
    }

    fn step_day(&mut self) {
        let prev = self.ages;
        for (age, count) in prev.into_iter().enumerate() {
            match age {
                0 => {
                    self.ages[8] = count;
                    self.ages[6] = count;
                }
                7 => self.ages[6] += count,
                _ => self.ages[age - 1] = count,
            }
        }
    }

    fn reset(&mut self) {
        for age in self.ages.iter_mut() {
            *age = 0;
        }
        for age in &self.starting {
            self.ages[*age] += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            ages: [0, 1, 1, 2, 1, 0, 0, 0, 0],
            starting: vec![3, 4, 3, 1, 2],
            ..Default::default()
        };
        assert_eq!(5934, day.simulate(80));
        day.reset();
        assert_eq!(26984457539, day.simulate(256));
    }
}
