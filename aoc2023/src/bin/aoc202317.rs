use std::{collections::HashMap, fmt::Display};

use aoc::{
    runner::{output, run_solution, Runner},
    search::{a_star, Graph, Searcher, Weighted},
    Dir, Point,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/day17.txt".into(),
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
        output(self.get_path(Point(1, 3)))
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.get_path(Point(4, 10)))
    }
}

impl AocDay {
    fn get_path(&self, limits: Point<usize>) -> usize {
        a_star(
            &Node {
                pos: Point(0, 0),
                dir: Dir::East,
                steps: 0,
                limits,
            },
            &self.map,
            |node| (self.map.height() - node.pos.0) + (self.map.width() - node.pos.1),
        )
        .unwrap_or_default()
        .1
    }
}

#[derive(Debug, Default)]
struct Map {
    grid: HashMap<Point<usize>, usize>,
    size: Point<usize>,
}

impl Graph for Map {
    fn value(&self, row: usize, col: usize) -> usize {
        self.grid[&Point(row, col)]
    }

    fn height(&self) -> usize {
        self.size.0
    }
    fn width(&self) -> usize {
        self.size.1
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    pos: Point<usize>,
    dir: Dir,
    steps: usize,
    limits: Point<usize>,
}

impl Searcher<Map> for Node {
    fn moves(&self, map: &Map) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut neighbors = Vec::new();
        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            // Don't back track
            if dir
                == match self.dir {
                    Dir::North => Dir::South,
                    Dir::South => Dir::North,
                    Dir::East => Dir::West,
                    Dir::West => Dir::East,
                }
            {
                continue;
            }

            // Dont' go off the map
            if (self.pos.0 == 0 && dir == Dir::North)
                || (self.pos.1 == 0 && dir == Dir::West)
                || (self.pos.0 == map.height() - 1 && dir == Dir::South)
                || (self.pos.1 == map.width() - 1 && dir == Dir::East)
            {
                continue;
            }
            let pos = dir.delta(&self.pos);
            if dir == self.dir && self.steps < self.limits.1 {
                neighbors.push(Self {
                    pos,
                    dir,
                    steps: self.steps + 1,
                    limits: self.limits,
                });
            } else if self.steps == 0 || (dir != self.dir && self.steps >= self.limits.0) {
                neighbors.push(Self {
                    pos,
                    dir,
                    steps: 1,
                    limits: self.limits,
                })
            }
        }
        neighbors
    }

    fn is_done(&self, map: &Map) -> bool {
        let target = Point(map.height() - 1, map.width() - 1);
        self.pos == target && self.steps >= self.limits.0
    }
}

impl Weighted<Map> for Node {
    fn weight(&self, graph: &Map) -> usize {
        graph.value(self.pos.0, self.pos.1)
    }
}

impl From<Vec<Vec<char>>> for Map {
    fn from(value: Vec<Vec<char>>) -> Self {
        let size = Point(value.len(), value[0].len());
        let grid = HashMap::from_iter(value.into_iter().enumerate().flat_map(|(row, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col, chr)| (Point(row, col), (chr as u8 - b'0') as usize))
                .collect::<Vec<_>>()
        }));
        Self { grid, size }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                write!(f, "{}", self.grid[&Point(row, col)])?;
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

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 102;
        let actual = day.get_path(Point(1, 3));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 94;
        let actual = day.get_path(Point(4, 10));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_example2() {
        let mut day = AocDay {
            input: "111111111111
999999999991
999999999991
999999999991
999999999991"
                .into(),
            ..Default::default()
        };
        day.parse();
        let expected = 71;
        let actual = day.get_path(Point(4, 10));
        assert_eq!(expected, actual);
    }
}
