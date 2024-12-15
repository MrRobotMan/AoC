use std::{collections::HashMap, fmt::Display};

use aoc::{
    reader::{contents, read_grid_to_map, read_line},
    runner::{output, Runner},
    Vec2D,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    warehouse: String,
    instructions: Vec<Vec2D<i64>>,
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
        (2024, 15)
    }

    fn parse(&mut self) {
        let contents = contents(&self.input);
        let (grid, inst) = contents.split_once("\n\n").unwrap();
        self.warehouse = grid.into();
        self.instructions = read_line(inst)
            .iter()
            .copied()
            .map(|c| c.try_into().unwrap())
            .collect();
    }

    fn part1(&mut self) -> String {
        let mut warehouse: Warehouse = self.warehouse.as_str().into();
        for inst in &self.instructions {
            warehouse.step(*inst, false);
        }
        output(warehouse.goods_score())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug, Clone, Default)]
struct Warehouse {
    robot: Vec2D<i64>,
    layout: HashMap<Vec2D<i64>, char>,
    rows: i64,
    cols: i64,
}

impl Warehouse {
    fn valid_move_small(&self, inst: Vec2D<i64>) -> bool {
        let mut next_move = self.robot + inst;
        while let Some(chr) = self.layout.get(&next_move) {
            if *chr == '#' {
                return false;
            }
            next_move += inst;
        }
        true
    }

    fn step(&mut self, inst: Vec2D<i64>, big: bool) {
        if (!big && !self.valid_move_small(inst)) || (big && !self.valid_move_big(inst)) {
            return;
        }
        let mut next_move = self.robot + inst;
        self.robot += inst;
        let mut to_insert = vec![];
        while let Some(chr) = self.layout.remove(&next_move) {
            next_move += inst;
            to_insert.push((next_move, chr));
        }
        for (p, c) in to_insert {
            self.layout.insert(p, c);
        }
    }

    fn goods_score(&self) -> i64 {
        self.layout.iter().fold(0, |acc, (loc, _box)| {
            if *_box == 'O' {
                acc + (100 * loc.0) + loc.1
            } else {
                acc
            }
        })
    }
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let mut warehouse = Self::default();
        for (point, chr) in read_grid_to_map(value) {
            match chr {
                '#' | 'O' => {
                    warehouse
                        .layout
                        .insert(Vec2D(point.0 as i64, point.1 as i64), chr);
                }
                '@' => warehouse.robot = Vec2D(point.0 as i64, point.1 as i64),
                _ => (),
            }
            warehouse.rows = warehouse.rows.max(point.0 as i64 + 1);
            warehouse.cols = warehouse.cols.max(point.1 as i64 + 1);
        }
        warehouse
    }
}
impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", {
                    if Vec2D(row, col) == self.robot {
                        '@'
                    } else {
                        match self.layout.get(&Vec2D(row, col)) {
                            None => '.',
                            Some(c) => *c,
                        }
                    }
                })?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        let warehouse = read_grid_to_map(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        )
        .iter()
        .filter_map(|(p, c)| {
            if ['.', '@'].contains(c) {
                None
            } else {
                Some((Vec2D(p.0 as i64, p.1 as i64), *c))
            }
        })
        .collect::<HashMap<_, _>>();
        let instructions = "<^^>>>vv<v>>v<<"
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<Vec2D<i64>>>();
        let mut day = AocDay::new(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv
<v>>v<<",
        );
        day.parse();
        assert_eq!(warehouse, day.warehouse.layout);
        assert_eq!(instructions, day.instructions);
        assert_eq!(Vec2D(2, 2), day.warehouse.robot);
    }

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv
<v>>v<<",
        );
        day.parse();
        let expected = "2028";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
