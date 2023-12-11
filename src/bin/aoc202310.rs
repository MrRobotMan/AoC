use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    grid: HashMap<(i32, i32), Pipe>,
    mainloop: HashSet<(i32, i32)>,
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

    fn part1(&mut self) -> Vec<String> {
        output((self.mainloop.len() + 1) / 2)
    }

    fn part2(&mut self) -> Vec<String> {
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
    fn make_loop(&mut self) {
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
        match self.grid[&current] {
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
enum Pipe {
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
            mainloop: HashSet::new(),
        };
        day.make_loop();

        let expected = 16;
        let actual = day.mainloop.len();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let mut grid = HashMap::new();
        let mut start = (0, 0);
        for (row, line) in ["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"]
            .iter()
            .enumerate()
        {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    'S' => start = (row as i32, col as i32),
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
            mainloop: HashSet::new(),
        };
        day.make_loop();

        let expected = 8;
        let actual = day.part1()[0].parse::<i32>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_simplified() {
        let mut grid = HashMap::new();
        let mut start = (0, 0);
        for (row, line) in [
            "...........".to_string(),
            ".S-------7.".to_string(),
            ".|F-----7|.".to_string(),
            ".||OOOOO||.".to_string(),
            ".||OOOOO||.".to_string(),
            ".|L-7OF-J|.".to_string(),
            ".|II|O|II|.".to_string(),
            ".L--JOL--J.".to_string(),
            ".....O.....".to_string(),
        ]
        .iter()
        .enumerate()
        {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    'S' => start = (row as i32, col as i32),
                    c => {
                        grid.insert((row as i32, col as i32), c.into());
                    }
                }
            }
        }
        let mut day = AocDay {
            grid,
            start,
            size: (9, 11),
            mainloop: HashSet::new(),
        };
        day.make_loop();

        let expected = 4;
        let actual = day.part2()[0].parse::<i32>().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_part2() {
        let mut grid = HashMap::new();
        let mut start = (0, 0);
        for (row, line) in [
            "FF7FSF7F7F7F7F7F---7".to_string(),
            "L|LJ||||||||||||F--J".to_string(),
            "FL-7LJLJ||||||LJL-77".to_string(),
            "F--JF--7||LJLJ7F7FJ-".to_string(),
            "L---JF-JLJ.||-FJLJJ7".to_string(),
            "|F|F-JF---7F7-L7L|7|".to_string(),
            "|FFJF7L7F-JF7|JL---7".to_string(),
            "7-L-JL7||F7|L7F-7F7|".to_string(),
            "L.L7LFJ|||||FJL7||LJ".to_string(),
            "L7JLJL-JLJLJL--JLJ.L".to_string(),
        ]
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
            size: (10, 20),
            mainloop: HashSet::new(),
        };
        day.make_loop();

        let expected = 10;
        let actual = day.part2()[0].parse::<i32>().unwrap();
        assert_eq!(expected, actual);
    }
}
