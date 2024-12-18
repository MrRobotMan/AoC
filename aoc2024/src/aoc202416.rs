use std::{collections::HashSet, str::FromStr};

use aoc::{
    measure::CARDINALS,
    read_grid_to_map,
    runner::{output, Runner},
    search::{dijkstra, Weighted},
    Vec2D,
};
use pathfinding::prelude::astar_bag;

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    maze: Maze,
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
        (2024, 16)
    }

    fn parse(&mut self) {
        self.maze = self.input.parse().unwrap();
    }

    fn part1(&mut self) -> String {
        output(self.maze.score())
    }

    fn part2(&mut self) -> String {
        output(self.maze.all_paths())
    }
}

#[derive(Debug, Default)]
struct Maze {
    nodes: HashSet<Vec2D<i64>>,
    start: Vec2D<i64>,
    end: Vec2D<i64>,
}

impl Maze {
    fn score(&self) -> usize {
        match dijkstra(&(self.start, Vec2D(0, 1)), self) {
            None => 0,
            Some(graph) => graph
                .iter()
                .filter_map(|(n, v)| if n.0 == self.end { Some(*v) } else { None })
                .next()
                .unwrap_or(0),
        }
    }

    fn all_paths(&self) -> usize {
        let Some((paths, _)) = astar_bag(
            &(self.start, Vec2D(0, 1)),
            |node| self.weighted_moves(node),
            |node| node.0.manhatten(&self.end) as usize,
            |node| node.0 == self.end,
        ) else {
            panic!("No Paths!")
        };
        let found = paths
            .flat_map(|p| p.iter().map(|n| n.0).collect::<HashSet<_>>())
            .collect::<HashSet<_>>();
        found.len()
    }

    fn weighted_moves(&self, cur: &Node) -> Vec<(Node, usize)> {
        CARDINALS
            .iter()
            .filter_map(|dir| {
                self.nodes
                    .get(&(cur.0 + *dir))
                    .map(|n| ((*n, *dir), self.weight(cur, &(*n, *dir))))
            })
            .collect()
    }
}

type Node = (Vec2D<i64>, Vec2D<i64>);

impl Weighted for Maze {
    type Node = Node;
    fn weight(&self, cur: &Self::Node, next: &Self::Node) -> usize {
        match cur.1 - next.1 {
            Vec2D(-1, -1) | Vec2D(1, -1) | Vec2D(-1, 1) | Vec2D(1, 1) => 1001,
            Vec2D(0, 0) => 1,
            Vec2D(2, 0) | Vec2D(-2, 0) | Vec2D(0, -2) | Vec2D(0, 2) => 2001, // 180°
            _ => unreachable!("Bad math"),
        }
    }

    fn moves(&self, cur: &Self::Node) -> Vec<Self::Node> {
        CARDINALS
            .iter()
            .filter_map(|dir| self.nodes.get(&(cur.0 + *dir)).map(|n| (*n, *dir)))
            .collect()
    }

    fn is_done(&self, node: &Self::Node) -> bool {
        node.0 == self.end
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maze = Self::default();
        for ((r, c), ch) in read_grid_to_map(s) {
            match ch {
                '.' => {
                    maze.nodes.insert(Vec2D(r as i64, c as i64));
                }
                'S' => {
                    maze.start = Vec2D(r as i64, c as i64);
                    maze.nodes.insert(Vec2D(r as i64, c as i64));
                }
                'E' => {
                    maze.end = Vec2D(r as i64, c as i64);
                    maze.nodes.insert(Vec2D(r as i64, c as i64));
                }
                _ => (),
            }
        }
        Ok(maze)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let maze = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
        .parse::<Maze>()
        .unwrap();
        let expected = 7036;
        let actual = maze.score();
        assert_eq!(expected, actual);
    }
}
