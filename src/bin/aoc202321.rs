use std::collections::HashMap;

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day21.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    garden: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 21)
    }

    fn parse(&mut self) {
        let lines = aoc::read_grid(&self.input);
        self.width = lines[0].len();
        self.height = lines[1].len();
        self.garden = HashMap::from_iter(lines.into_iter().enumerate().flat_map(|(row, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(|(col, ch)| {
                    if ch == '.' {
                        None
                    } else {
                        Some(((row, col), ch))
                    }
                })
                .collect::<Vec<_>>()
        }));
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}
