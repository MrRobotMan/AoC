use std::{collections::HashMap, fmt::Display};

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day17.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    map: Map,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 17)
    }

    fn parse(&mut self) {
        self.map = aoc::read_grid(&self.input).into();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, Default)]
struct Map {
    grid: HashMap<(usize, usize), usize>,
    size: (usize, usize),
}

impl From<Vec<Vec<char>>> for Map {
    fn from(value: Vec<Vec<char>>) -> Self {
        let size = (value.len(), value[0].len());
        let grid = HashMap::from_iter(value.into_iter().enumerate().flat_map(|(row, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col, chr)| ((row, col), (chr as u8 - b'0') as usize))
                .collect::<Vec<_>>()
        }));
        Self { grid, size }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                write!(f, "{}", self.grid[&(row, col)])?;
            }
            if row != self.size.0 - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_parse() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        println!("{}", day.map);
        let expected = INPUT;
        let actual = day.map.to_string();
        assert_eq!(expected, actual);
    }
}
