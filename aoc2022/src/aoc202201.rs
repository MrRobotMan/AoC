use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    calories: Vec<Vec<i64>>,
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
        (2022, 1)
    }

    fn parse(&mut self) {
        self.calories = aoc::read_number_records(&self.input);
    }

    fn part1(&mut self) -> String {
        output(
            self.calories
                .iter()
                .map(|elf| elf.iter().sum::<i64>())
                .max()
                .unwrap(),
        )
    }

    fn part2(&mut self) -> String {
        let mut top = self
            .calories
            .iter()
            .map(|elf| elf.iter().sum::<i64>())
            .collect::<Vec<_>>();
        top.sort_by(|a, b| b.cmp(a));
        output(top.iter().take(3).sum::<i64>())
    }
}
