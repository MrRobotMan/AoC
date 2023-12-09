use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    histories: Vec<Vec<i64>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 9)
    }

    fn parse(&mut self) {
        self.histories = aoc::read_lines("inputs/2023/day09.txt")
            .iter()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();
        for hist in &self.histories[..10] {
            println!("{hist:?}");
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}
