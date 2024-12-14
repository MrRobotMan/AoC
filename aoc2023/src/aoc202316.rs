use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc::{
    runner::{output, Runner},
    Dir, Vec2D,
};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub grid: HashMap<Vec2D<i32>, Cave>,
    pub size: Vec2D<i32>,
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
        (2023, 16)
    }

    fn parse(&mut self) {
        for (row, line) in aoc::read_lines(&self.input).iter().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                self.grid.insert(Vec2D(row as i32, col as i32), chr.into());
                self.size = Vec2D(row as i32 + 1, col as i32 + 1);
            }
        }
    }

    fn part1(&mut self) -> String {
        output(self.light_path(Vec2D(0, 0), Dir::East))
    }

    fn part2(&mut self) -> String {
        let mut res = 0;
        for row in 0..self.size.0 {
            // First col going east
            res = res.max(self.light_path(Vec2D(row, 0), Dir::East));

            // Last col going west
            res = res.max(self.light_path(Vec2D(row, self.size.1 - 1), Dir::West));
        }
        for col in 0..self.size.1 {
            // First row going south
            res = res.max(self.light_path(Vec2D(0, col), Dir::South));

            // Last row going north
            res = res.max(self.light_path(Vec2D(self.size.0 - 1, col), Dir::North));
        }
        output(res)
    }
}

impl AocDay {
    fn light_path(&mut self, start_point: Vec2D<i32>, dir: Dir) -> usize {
        let mut to_visit = vec![(start_point, dir)];
        let mut visited = HashSet::new();
        let mut energized = HashSet::new();
        while let Some((pos, dir)) = to_visit.pop() {
            if visited.insert((pos, dir)) {
                energized.insert(pos);
                if let Some(cave) = self.grid.get(&pos) {
                    for dir in cave.do_step(dir) {
                        let new_pos = dir.delta(&pos);
                        if new_pos.0 >= 0
                            && new_pos.0 < self.size.0
                            && new_pos.1 >= 0
                            && new_pos.1 < self.size.1
                        {
                            to_visit.push((new_pos, dir));
                        };
                    }
                };
            };
        }
        energized.len()
    }
}

impl Display for AocDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                write!(f, "{}", self.grid[&Vec2D(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Cave {
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
