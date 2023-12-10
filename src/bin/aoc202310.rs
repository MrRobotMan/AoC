use std::{collections::HashMap, fmt::Display};

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    grid: HashMap<(i32, i32), Pipe>,
    mainloop: HashMap<(i32, i32), Pipe>,
    start: (i32, i32),
    size: (i32, i32),
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 10)
    }

    fn parse(&mut self) {
        let lines = aoc::read_lines("inputs/2023/day10.txt");
        self.size = (lines.len() as i32, lines[0].len() as i32);
        for (row, line) in lines.into_iter().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    'S' => self.start = (row as i32, col as i32),
                    '.' => (),
                    c => {
                        self.grid.insert((row as i32, col as i32), c.into());
                    }
                }
            }
        }
        self.make_loop();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl AocDay {
    fn make_loop(&mut self) {
        let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)] // Above, right, below, left
            .iter()
            .map(|(row, col)| self.grid.get(&(self.start.0 + row, self.start.1 + col)))
            .collect::<Vec<_>>();
        let mut matches = Vec::new();
        if let (Some(p1), Some(p2)) = (neighbors[0], neighbors[1]) {
            // above, right
            if matches!(
                format!("{p1}{p2}").as_str(),
                "|J" | "|7" | "|-" | "FJ" | "F7" | "F-" | "7J" | "77" | "7-"
            ) {
                matches.push((0, 1, Pipe::NeElbow))
            }
        }

        if let (Some(p1), Some(p2)) = (neighbors[0], neighbors[3]) {
            // above, left
            if matches!(
                format!("{p1}{p2}").as_str(),
                "|F" | "|L" | "|-" | "FF" | "FL" | "F-" | "7F" | "7L" | "7-"
            ) {
                matches.push((0, 3, Pipe::NwElbow))
            }
        }

        if let (Some(p1), Some(p2)) = (neighbors[0], neighbors[2]) {
            // above, below
            if matches!(
                format!("{p1}{p2}").as_str(),
                "|J" | "|L" | "||" | "FJ" | "FL" | "F|" | "7J" | "7L" | "7|"
            ) {
                matches.push((0, 2, Pipe::Vertical))
            }
        }

        if let (Some(p1), Some(p2)) = (neighbors[3], neighbors[2]) {
            // left, below
            if matches!(
                format!("{p1}{p2}").as_str(),
                "-J" | "-L" | "-|" | "FJ" | "FL" | "F|" | "LJ" | "LL" | "L|"
            ) {
                matches.push((3, 2, Pipe::SwElbow))
            }
        }

        if let (Some(p1), Some(p2)) = (neighbors[3], neighbors[1]) {
            // left, right
            if matches!(
                format!("{p1}{p2}").as_str(),
                "-J" | "-7" | "--" | "FJ" | "F7" | "F-" | "LJ" | "L7" | "L-"
            ) {
                matches.push((3, 1, Pipe::SeElbow))
            }
        }

        if let (Some(p1), Some(p2)) = (neighbors[2], neighbors[1]) {
            // below, right
            if matches!(
                format!("{p1}{p2}").as_str(),
                "|J" | "|7" | "|-" | "JJ" | "J7" | "J-" | "LJ" | "L7" | "L-"
            ) {
                matches.push((2, 1, Pipe::Horizontal))
            }
        }

        if matches.len() == 1 {
            todo!()
        } else {
            panic!("Multiple Options for loop start");
        }
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

#[derive(Debug, Copy, Clone)]
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
    fn test_loop_builder() {
        let mut grid = HashMap::new();
        let mut start = (0, 0);
        for (row, line) in ["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"]
            .iter()
            .enumerate()
        {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    'S' => start = (row as i32, col as i32),
                    '.' => (),
                    c => {
                        grid.insert((row as i32, col as i32), c.into());
                    }
                }
            }
        }
        let mut day = AocDay {
            grid,
            start,
            size: (5, 5),
            mainloop: HashMap::new(),
        };
        day.make_loop();
        let expected = 16;
        let actual = day.mainloop.len();
        assert_eq!(expected, actual);
    }
}
