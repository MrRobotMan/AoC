use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub grid: HashMap<(i32, i32), Pipe>,
    pub mainloop: HashSet<(i32, i32)>,
    pub start: (i32, i32),
    pub size: (i32, i32),
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 10)
    }

    fn parse(&mut self) {
        let lines = aoc::read_lines(&self.input);
        self.size = (lines.len() as i32, lines[0].len() as i32);
        for (row, line) in lines.into_iter().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    'S' => self.start = (row as i32, col as i32),
                    c => {
                        if !matches!(c.into(), Pipe::Empty) {
                            self.grid.insert((row as i32, col as i32), c.into());
                        }
                    }
                }
            }
        }
        self.make_loop();
    }

    fn part1(&mut self) -> String {
        output((self.mainloop.len() + 1) / 2)
    }

    fn part2(&mut self) -> String {
        let mut inside = false;
        let mut count = 0;
        for row in 0..self.size.0 {
            let mut cur = Pipe::Empty;
            for col in 0..self.size.1 {
                match self.mainloop.get(&(row, col)) {
                    None => {
                        if inside {
                            count += 1
                        }
                    }
                    Some(point) => {
                        match self.grid[point] {
                            Pipe::Vertical => inside = !inside,
                            Pipe::Horizontal => (), // Do nothing. Scanning across row. Change at corners.
                            Pipe::NeElbow => cur = Pipe::NeElbow,
                            Pipe::NwElbow => {
                                if matches!(cur, Pipe::SeElbow) {
                                    inside = !inside
                                }
                            }
                            Pipe::SeElbow => cur = Pipe::SeElbow,
                            Pipe::SwElbow => {
                                if matches!(cur, Pipe::NeElbow) {
                                    inside = !inside
                                }
                            }
                            Pipe::Empty => panic!("Somehow put an empty tile on the loop"),
                        }
                    }
                }
            }
        }
        output(count)
    }
}

impl AocDay {
    pub fn make_loop(&mut self) {
        let offsets = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // Above, right, below, left
        let neighbors = offsets
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
                matches.push((3, 1, Pipe::Horizontal))
            }
        }

        if let (Some(p1), Some(p2)) = (neighbors[2], neighbors[1]) {
            // below, right
            if matches!(
                format!("{p1}{p2}").as_str(),
                "|J" | "|7" | "|-" | "JJ" | "J7" | "J-" | "LJ" | "L7" | "L-"
            ) {
                matches.push((2, 1, Pipe::SeElbow))
            }
        }
        if matches.len() == 1 {
            let (start_node, pipe) = (offsets[matches[0].1], matches[0].2);
            self.grid.insert(self.start, pipe);
            let mut current = (self.start.0 + start_node.0, self.start.1 + start_node.1);
            let mut prev = self.start;
            while current != self.start {
                self.mainloop.insert(prev);
                (prev, current) = (current, self.get_next(&prev, &current));
            }
            self.mainloop.insert(prev);
        } else {
            panic!("Multiple Options for loop start");
        }
    }

    fn get_next(&self, prev: &(i32, i32), current: &(i32, i32)) -> (i32, i32) {
        let delta_row = current.0 - prev.0;
        let delta_col = current.1 - prev.1;
        match self.grid[current] {
            Pipe::Vertical => (current.0 + delta_row, current.1),
            Pipe::Horizontal => (current.0, current.1 + delta_col),
            Pipe::NeElbow => {
                if delta_row == 0 {
                    (current.0 - 1, current.1) // West to Up
                } else {
                    (current.0, current.1 + 1) // Down to East
                }
            }
            Pipe::NwElbow => {
                if delta_row == 0 {
                    (current.0 - 1, current.1) // East to Up
                } else {
                    (current.0, current.1 - 1) // Down to West
                }
            }
            Pipe::SwElbow => {
                if delta_row == 0 {
                    (current.0 + 1, current.1) // East to Down
                } else {
                    (current.0, current.1 - 1) // Up to West
                }
            }
            Pipe::SeElbow => {
                if delta_row == 0 {
                    (current.0 + 1, current.1) // West to Down
                } else {
                    (current.0, current.1 + 1) // Up to East
                }
            }
            Pipe::Empty => panic!("trying to visit an empty tile"),
        }
    }
}

impl Display for AocDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
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
pub enum Pipe {
    Vertical,   // | N-S
    Horizontal, // - E-W
    NeElbow,    // L Elbow connecting North to East
    NwElbow,    // J Elbow connecting North to West
    SwElbow,    // 7 Elbow connecting South to West
    SeElbow,    // F Elbow connecting South to East
    Empty,
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
            _ => Pipe::Empty,
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
                Pipe::Empty => '.',
            }
        )
    }
}
