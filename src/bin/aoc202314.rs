use std::{collections::HashMap, fmt::Display};

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day14.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    grid: HashMap<(usize, usize), Rock>,
    size: (usize, usize),
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 14)
    }

    fn parse(&mut self) {
        let lines = aoc::read_grid(&self.input);
        self.size = (lines.len(), lines[0].len());
        for (row, line) in lines.into_iter().enumerate() {
            for (col, chr) in line.into_iter().enumerate() {
                self.grid.insert(
                    (row, col),
                    match chr {
                        'O' => Rock::Round,
                        '#' => Rock::Square,
                        '.' => Rock::None,
                        c => panic!("Unknown item {c}"),
                    },
                );
            }
        }

        println!("{}", self);
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl Display for AocDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                write!(f, "{}", self.grid[&(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Default)]
enum Rock {
    Round,
    Square,
    #[default]
    None,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rock::Round => 'O',
                Rock::Square => '#',
                Rock::None => '.',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 136;
        let actual = day.part1()[0].parse::<i32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
