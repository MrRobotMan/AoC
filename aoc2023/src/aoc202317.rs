use std::{collections::HashMap, fmt::Display};

use aoc::{
    runner::{output, Runner},
    Dir, Point,
};

use pathfinding::directed::dijkstra::dijkstra;

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub map: Map,
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
    pub fn get_path(&self, limits: Point<usize>) -> usize {
        dijkstra(
            &Node {
                pos: Point(0, 0),
                dir: Dir::East,
                steps: 0,
                limits,
            },
            |node| self.map.moves(node),
            |node| self.map.is_done(node),
        )
        .unwrap()
        .1
    }
}

#[derive(Debug, Default)]
pub struct Map {
    grid: HashMap<Point<usize>, usize>,
    size: Point<usize>,
}

impl Map {
    fn moves(&self, node: &Node) -> Vec<(Node, usize)> {
        let mut neighbors = Vec::new();
        let height = self.size.0 - 1;
        let width = self.size.1 - 1;
        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            // Don't back track
            if dir
                == match node.dir {
                    Dir::North => Dir::South,
                    Dir::South => Dir::North,
                    Dir::East => Dir::West,
                    Dir::West => Dir::East,
                }
            {
                continue;
            }

            // Dont' go off the map
            if (node.pos.0 == 0 && dir == Dir::North)
                || (node.pos.1 == 0 && dir == Dir::West)
                || (node.pos.0 == height && dir == Dir::South)
                || (node.pos.1 == width && dir == Dir::East)
            {
                continue;
            }
            let pos = dir.delta(&node.pos);
            if dir == node.dir && node.steps < node.limits.1 {
                neighbors.push((
                    Node {
                        pos,
                        dir,
                        steps: node.steps + 1,
                        limits: node.limits,
                    },
                    self.grid[&pos],
                ));
            } else if node.steps == 0 || (dir != node.dir && node.steps >= node.limits.0) {
                neighbors.push((
                    Node {
                        pos,
                        dir,
                        steps: 1,
                        limits: node.limits,
                    },
                    self.grid[&pos],
                ))
            }
        }
        neighbors
    }

    fn is_done(&self, node: &Node) -> bool {
        let target = Point(self.size.0 - 1, self.size.1 - 1);
        node.pos == target && node.steps >= node.limits.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    pos: Point<usize>,
    dir: Dir,
    steps: usize,
    limits: Point<usize>,
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
