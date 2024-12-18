use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc::{
    measure::CARDINALS,
    read_grid_to_map,
    runner::{output, Runner},
    search::{dijkstra, Weighted},
    Vec2D,
};

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
        output("Unsolved")
    }
}

#[derive(Debug, Default)]
struct Maze {
    nodes: HashSet<Vec2D<i64>>,
    start: Vec2D<i64>,
    end: Vec2D<i64>,
    rows: usize,
    cols: usize,
}

impl Maze {
    fn score(&self) -> usize {
        match dijkstra(&(self.start, Vec2D(0, 1)), self) {
            None => 0,
            Some(graph) => {
                // self.show(&graph);
                graph
                    .iter()
                    .filter_map(|(n, v)| if n.0 == self.end { Some(*v) } else { None })
                    .next()
                    .unwrap_or(0)
            }
        }
    }

    fn show(&self, graph: &HashMap<(Vec2D<i64>, Vec2D<i64>), usize>) {
        let path = self.path(graph);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let point = Vec2D(row as i64, col as i64);
                if point == self.start {
                    print!("S");
                } else if point == self.end {
                    print!("E");
                } else if path.contains_key(&point) {
                    print!("{}", path[&point])
                } else {
                    print!(
                        "{}",
                        match self.nodes.get(&point) {
                            None => '#',
                            Some(_) => '.',
                        }
                    )
                }
            }
            println!();
        }
        println!("{}", path.len());
    }

    fn path(&self, graph: &HashMap<(Vec2D<i64>, Vec2D<i64>), usize>) -> HashMap<Vec2D<i64>, char> {
        let mut visited = HashMap::new();
        let mut cur = self.end;
        let end = graph
            .keys()
            .find(|n| n.0 == self.end)
            .map(|v| (v.0, dir_to_char(v.1)))
            .unwrap();
        visited.insert(end.0, end.1);
        while cur != self.start {
            let found = CARDINALS
                .iter()
                .map(|d| {
                    graph
                        .iter()
                        .filter(|(n, _)| n.0 == cur - *d)
                        .min_by_key(|v| v.1)
                        .unwrap_or((&(Vec2D(0, 0), Vec2D(1, 0)), &usize::MAX))
                })
                .min_by_key(|v| v.1)
                .unwrap()
                .0;
            cur = found.0;
            visited.insert(found.0, dir_to_char(found.1));
        }
        visited
    }
}

fn dir_to_char(dir: Vec2D<i64>) -> char {
    match dir {
        Vec2D(-1, 0) => '^',
        Vec2D(0, 1) => '>',
        Vec2D(1, 0) => 'v',
        Vec2D(0, -1) => '<',
        _ => unreachable!(),
    }
}

impl Weighted for Maze {
    type Node = (Vec2D<i64>, Vec2D<i64>);
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
            maze.rows = maze.rows.max(r + 1);
            maze.cols = maze.cols.max(c + 1);
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
