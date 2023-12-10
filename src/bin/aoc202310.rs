use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    // Add some data structure
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 10)
    }

    fn parse(&mut self) {
        // Parse the input
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}
        