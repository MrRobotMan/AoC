use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc::{
    runner::{output, run_solution, Runner},
    Dir,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day16.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    grid: HashMap<(i32, i32), Cave>,
    size: (i32, i32),
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 16)
    }

    fn parse(&mut self) {
        for (row, line) in aoc::read_lines(&self.input).iter().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                self.grid.insert((row as i32, col as i32), chr.into());
                self.size = (row as i32 + 1, col as i32 + 1);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut visited = HashSet::new();
        self.light_path((0, 0), Dir::East, &mut visited);
        let energized = visited.iter().map(|v| v.0).collect::<HashSet<_>>();
        output(energized.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        let mut visited = HashSet::new();

        for row in 0..self.size.0 {
            // First col going east
            visited.clear();
            // energized.clear();
            self.light_path((row, 0), Dir::East, &mut visited);
            res = res.max(visited.iter().map(|v| v.0).collect::<HashSet<_>>().len());

            // Last col going west
            visited.clear();
            // energized.clear();
            self.light_path((row, self.size.1 - 1), Dir::West, &mut visited);
            res = res.max(visited.iter().map(|v| v.0).collect::<HashSet<_>>().len());
        }
        for col in 0..self.size.1 {
            // First row going south
            visited.clear();
            // energized.clear();
            self.light_path((0, col), Dir::South, &mut visited);
            res = res.max(visited.iter().map(|v| v.0).collect::<HashSet<_>>().len());

            // Last row going north
            visited.clear();
            // energized.clear();
            self.light_path((self.size.0 - 1, col), Dir::North, &mut visited);
            res = res.max(visited.iter().map(|v| v.0).collect::<HashSet<_>>().len());
        }
        output(res)
    }
}

impl AocDay {
    fn light_path(&mut self, pos: (i32, i32), dir: Dir, visited: &mut HashSet<((i32, i32), Dir)>) {
        if visited.insert((pos, dir)) {
            if let Some(cave) = self.grid.get(&pos) {
                for dir in cave.do_step(dir) {
                    let delta = dir.delta();
                    let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
                    if new_pos.0 < 0
                        || new_pos.0 >= self.size.0
                        || new_pos.1 < 0
                        || new_pos.1 >= self.size.1
                    {
                        continue;
                    }
                    self.light_path(new_pos, dir, visited);
                }
            }
        }
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

#[derive(Debug, Default, PartialEq)]
enum Cave {
    #[default]
    Empty,
    SwNeMirror, // /
    SeNwMirror, // \
    Vertical,   // |
    Horizontal, // -
}

impl Cave {
    fn do_step(&self, dir: Dir) -> Vec<Dir> {
        match (self, dir) {
            (Self::Empty, _) => vec![dir],
            (Self::SwNeMirror, Dir::North) => vec![Dir::East],
            (Self::SwNeMirror, Dir::South) => vec![Dir::West],
            (Self::SwNeMirror, Dir::East) => vec![Dir::North],
            (Self::SwNeMirror, Dir::West) => vec![Dir::South],
            (Self::SeNwMirror, Dir::North) => vec![Dir::West],
            (Self::SeNwMirror, Dir::South) => vec![Dir::East],
            (Self::SeNwMirror, Dir::East) => vec![Dir::South],
            (Self::SeNwMirror, Dir::West) => vec![Dir::North],
            (Self::Vertical, Dir::North | Dir::South) => vec![dir],
            (Self::Vertical, Dir::East | Dir::West) => vec![Dir::North, Dir::South],
            (Self::Horizontal, Dir::North | Dir::South) => vec![Dir::East, Dir::West],
            (Self::Horizontal, Dir::East | Dir::West) => vec![dir],
        }
    }
}

impl From<char> for Cave {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Cave::SwNeMirror,
            '\\' => Cave::SeNwMirror,
            '|' => Cave::Vertical,
            '-' => Cave::Horizontal,
            _ => panic!("Unknown character {value}"),
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cave::Empty => '.',
                Cave::SwNeMirror => '/',
                Cave::SeNwMirror => '\\',
                Cave::Vertical => '|',
                Cave::Horizontal => '-',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_parsing() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = (10, 10);
        let actual = day.size;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 46;
        let actual = day.part1()[0].parse::<i32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 51;
        let actual = day.part2()[0].parse::<i32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
