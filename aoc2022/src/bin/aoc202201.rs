use aoc::runner::{output, run_solution, Runner};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day01.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    calories: Vec<Vec<i64>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 1)
    }

    fn parse(&mut self) {
        self.calories = aoc::read_number_records(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.calories
                .iter()
                .map(|elf| elf.iter().sum::<i64>())
                .max()
                .unwrap(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut top = self
            .calories
            .iter()
            .map(|elf| elf.iter().sum::<i64>())
            .collect::<Vec<_>>();
        top.sort_by(|a, b| b.cmp(a));
        output(top.iter().take(3).sum::<i64>())
    }
}
