use std::{collections::HashSet, num::ParseIntError};

use aoc::{
    read_lines,
    runner::{output, Runner},
    Vec2D,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    dots: HashSet<Vec2D<i64>>, // Point in col, row (x, y)
    folds: Vec<Direction>,
    size: (i64, i64),
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn fold(&mut self, direction: Direction) {
        let points = match direction {
            Direction::Y(line) => (line..self.size.1)
                .flat_map(|row| (0..self.size.0).map(move |col| Vec2D(col, row)))
                .collect::<Vec<_>>(),
            Direction::X(column) => (column..self.size.0)
                .flat_map(|col| (0..self.size.1).map(move |row| Vec2D(col, row)))
                .collect::<Vec<_>>(),
        };
        for point in points {
            if self.dots.remove(&point) {
                let new = match direction {
                    // delta = X - x
                    // new = X - 2 Delta
                    // new = X - 2 (X - x)
                    // new = X - 2 X + 2 x
                    // new = 2 x - X
                    Direction::Y(row) => Vec2D(point.0, 2 * row - point.1),
                    Direction::X(col) => Vec2D(2 * col - point.0, point.1),
                };
                self.dots.insert(new);
            }
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2021, 13)
    }

    fn parse(&mut self) {
        let mut rows = 0;
        let mut cols = 0;
        let points = read_lines(&self.input);
        for point in points {
            match convert(&point) {
                Ok(dot) => {
                    cols = cols.max(dot.0);
                    rows = rows.max(dot.1);
                    self.dots.insert(dot);
                }
                Err(_) => {
                    let (dir, loc) = point.split_once('=').unwrap();
                    let loc = loc.parse().unwrap();
                    if dir.strip_suffix('x').is_some() {
                        self.folds.push(Direction::X(loc))
                    } else {
                        self.folds.push(Direction::Y(loc))
                    }
                }
            }
        }
        self.size = (cols, rows);
    }

    fn part1(&mut self) -> String {
        self.fold(self.folds[0]);
        output(self.dots.len())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn convert(input: &str) -> Result<Vec2D<i64>, ParseIntError> {
    let points = input
        .split(',')
        .map(|n| n.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    Ok(Vec2D(points[0], points[1]))
}

#[derive(Clone, Copy)]
enum Direction {
    X(i64),
    Y(i64),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fold_x() {
        // # . # . . | . . . # . => # # # . .
        // . . . . . | # . . . .    . . . . #
        let dots = HashSet::from([Vec2D(0, 0), Vec2D(2, 0), Vec2D(9, 0), Vec2D(6, 1)]);
        let mut day = AocDay {
            dots,
            size: (11, 2),
            ..Default::default()
        };
        day.fold(Direction::X(5));
        let expected = HashSet::from([Vec2D(0, 0), Vec2D(2, 0), Vec2D(1, 0), Vec2D(4, 1)]);
        let actual = day.dots;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fold_y() {
        // # . # . . | . . . # . => # # # . .
        // . . . . . | # . . . .    . . . . #
        let dots = HashSet::from([Vec2D(0, 0), Vec2D(2, 0), Vec2D(9, 0), Vec2D(6, 2)]);
        let mut day = AocDay {
            dots,
            size: (11, 3),
            ..Default::default()
        };
        day.fold(Direction::Y(1));
        let expected = HashSet::from([Vec2D(0, 0), Vec2D(2, 0), Vec2D(9, 0), Vec2D(6, 0)]);
        let actual = day.dots;
        assert_eq!(expected, actual);
    }
}
