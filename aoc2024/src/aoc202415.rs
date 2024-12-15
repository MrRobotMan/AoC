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
        let mut warehouse = Warehouse::new(&self.warehouse, false);
        for inst in &self.instructions {
            warehouse.step(*inst);
        }
        output(warehouse.goods_score())
    }

    fn part2(&mut self) -> String {
        let mut warehouse = Warehouse::new(&self.warehouse, true);
        for inst in &self.instructions {
            warehouse.step(*inst);
        }
        output(warehouse.goods_score())
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
    fn step(&mut self, inst: Vec2D<i64>) {
        let mut replace = vec![];
        let mut next_move = vec![self.robot + inst];
        while let Some(next) = next_move.pop() {
            match (self.layout.get(&next), inst) {
                (None, _) => (),
                (Some('#'), _) => return, // Would hit a wall. Leave.
                // Up / down large box left side. Add the right side.
                (Some('['), Vec2D(_, 0)) => {
                    next_move.push(next + inst);
                    next_move.push(next + inst + Vec2D(0, 1));
                    replace.push((next, (next + inst, '[')));
                    replace.push((next + Vec2D(0, 1), (next + inst + Vec2D(0, 1), ']')));
                }
                // Up / down large box right side. Add the left side.
                (Some(']'), Vec2D(_, 0)) => {
                    next_move.push(next + inst);
                    next_move.push(next + inst + Vec2D(0, -1));
                    replace.push((next, (next + inst, ']')));
                    replace.push((next + Vec2D(0, -1), (next + inst + Vec2D(0, -1), '[')));
                }
                // Left / right or small box
                (Some(c), _) | (Some(c), Vec2D(0, _)) | (Some(c), Vec2D(0, _))
                    if ['O', '[', ']'].contains(c) =>
                {
                    next_move.push(next + inst);
                    replace.push((next, (next + inst, *c)));
                }
                _ => unreachable!("Checking {next:?}"),
            }
        }
        for (remove, _) in &replace {
            self.layout.remove(remove);
        }
        for (_, (p, c)) in &replace {
            self.layout.insert(*p, *c);
        }
        self.robot += inst;
    }

    fn goods_score(&self) -> i64 {
        self.layout.iter().fold(0, |acc, (loc, goods)| {
            if ['O', '['].contains(goods) {
                acc + (100 * loc.0) + loc.1
            } else {
                acc
            }
        })
    }

    fn new(layout: &str, big: bool) -> Self {
        match big {
            true => Self::new_big(layout),
            false => Self::new_small(layout),
        }
    }

    fn new_small(layout: &str) -> Self {
        let mut warehouse = Self::default();
        for (point, chr) in read_grid_to_map(layout) {
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

    fn new_big(layout: &str) -> Self {
        let mut warehouse = Self::default();
        for (point, chr) in read_grid_to_map(layout) {
            match chr {
                '#' => {
                    warehouse
                        .layout
                        .insert(Vec2D(point.0 as i64, 2 * point.1 as i64), chr);
                    warehouse
                        .layout
                        .insert(Vec2D(point.0 as i64, 2 * point.1 as i64 + 1), chr);
                }
                'O' => {
                    warehouse
                        .layout
                        .insert(Vec2D(point.0 as i64, 2 * point.1 as i64), '[');
                    warehouse
                        .layout
                        .insert(Vec2D(point.0 as i64, 2 * point.1 as i64 + 1), ']');
                }
                '@' => warehouse.robot = Vec2D(point.0 as i64, 2 * point.1 as i64),
                _ => (),
            }
            warehouse.rows = warehouse.rows.max(point.0 as i64 + 1);
            warehouse.cols = warehouse.cols.max(2 * (point.1 as i64) + 1);
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
    #[test]
    fn test_example2() {
        let mut day = AocDay::new(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        );
        day.parse();
        let expected = "9021";
        let actual = day.part2();
        assert_eq!(expected, actual);
    }
}
