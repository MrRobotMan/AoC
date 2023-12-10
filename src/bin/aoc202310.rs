use std::{collections::HashMap, fmt::Display};

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    grid: HashMap<(usize, usize), Pipe>,
    start: (usize, usize),
    size: (usize, usize),
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 10)
    }

    fn parse(&mut self) {
        let lines = aoc::read_lines("inputs/2023/day10.txt");
        let _lines = vec![
            "-L|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ];
        self.size = (lines.len(), lines[0].len());
        for (row, line) in lines.into_iter().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    'S' => self.start = (row, col),
                    '.' => (),
                    c => {
                        self.grid.insert((row, col), c.into());
                    }
                }
            }
        }
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
        for col in 0..self.size.1 {
            for row in 0..self.size.0 {
                match self.grid.get(&(row, col)) {
                    None => write!(f, "{}", if (row, col) == self.start { 'S' } else { '.' })?,
                    Some(pipe) => write!(f, "{pipe}")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Pipe {
    Vertical,   //N-S
    Horizontal, //E-W
    NeElbow,    // Elbow connecting North to East
    NwElbow,    // Elbow connecting North to West
    SwElbow,    // Elbow connecting South to West
    SeElbow,    // Elbow connecting South to East
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NeElbow,
            'J' => Pipe::NwElbow,
            '7' => Pipe::SwElbow,
            'F' => Pipe::SeElbow,
            _ => panic!("Unknown pipe type {value}"),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::Vertical => '|',
                Pipe::Horizontal => '-',
                Pipe::NeElbow => 'L',
                Pipe::NwElbow => 'J',
                Pipe::SwElbow => '7',
                Pipe::SeElbow => 'F',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "-L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF";
        let expected = 10;
        let actual = 5;
        assert_eq!(expected, actual);
    }
}
