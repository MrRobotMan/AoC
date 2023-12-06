use aoc::{
    read_lines,
    runner::{output, run_solution, Runner},
};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    races: Vec<(i64, i64)>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn parse(&mut self) {
        let lines = read_lines("inputs/2023/day06.txt")
            .iter()
            .map(|l| {
                let (_, nums) = l.split_once(':').unwrap();
                nums.trim()
            })
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        self.races = lines[0]
            .iter()
            .zip(lines[1].iter())
            .map(|(t, d)| (*t, *d))
            .collect::<Vec<(i64, i64)>>();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}
