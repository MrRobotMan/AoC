use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay{input: "inputs/2023/day18.txt".into(), ..Default::default()};
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 18)
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
        