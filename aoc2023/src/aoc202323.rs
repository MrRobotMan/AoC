use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
    runner::{output, Runner},
    Dir, Vec2D,
};

use itertools::Itertools;
use pathfinding::directed::bfs::bfs;

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub trails: Vec<Vec<Tile>>,
    pub start: Vec2D<usize>,
    pub end: Vec2D<usize>,
    pub poi: HashSet<Vec2D<usize>>,
    pub height: usize,
    pub width: usize,
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
        (2023, 23)
    }

    fn parse(&mut self) {
        for line in aoc::read_grid(&self.input) {
            self.trails
                .push(line.iter().map(|c| c.into()).collect::<Vec<_>>());
        }
        self.height = self.trails.len();
        self.width = self.trails[0].len();
        self.start = Vec2D(
            0,
            self.trails[0]
                .iter()
                .position(|t| *t == Tile::Path)
                .unwrap(),
        );
        self.end = Vec2D(
            self.width - 1,
            self.trails[self.width - 1]
                .iter()
                .position(|t| *t == Tile::Path)
                .unwrap(),
        );

        self.poi.extend([self.start, self.end]);
        self.poi.extend(self.intersections());

        #[cfg(test)]
        self._dump();
    }

    fn part1(&mut self) -> String {
        let mut paths: HashMap<Vec2D<usize>, HashMap<Vec2D<usize>, usize>> = HashMap::new();
        for pair in self.poi.iter().permutations(2) {
            if let Some(path) = bfs(
                pair[0],
                |node| self.moves(node, true),
                |node| node == pair[1],
            ) {
                if HashSet::from_iter(path[1..path.len() - 1].iter().copied())
                    .intersection(&self.poi)
                    .count()
                    == 0
                {
                    // Path doesn't contain other points of interest.
                    paths
                        .entry(path[0])
                        .and_modify(|v| {
                            v.insert(*pair[1], path.len() - 1);
                        })
                        .or_insert(HashMap::from([(*pair[1], path.len() - 1)]));
                }
            };
        }
        output(self.bad_bfs(&paths))
    }

    fn part2(&mut self) -> String {
        let mut paths: HashMap<Vec2D<usize>, HashMap<Vec2D<usize>, usize>> = HashMap::new();
        for pair in self.poi.iter().permutations(2) {
            if let Some(path) = self.get_longest(pair[0], pair[1]) {
                paths
                    .entry(*pair[0])
                    .and_modify(|v| {
                        v.insert(*pair[1], path.len() - 1);
                    })
                    .or_insert(HashMap::from([(*pair[1], path.len() - 1)]));
            };
        }
        output(self.bad_bfs(&paths))
    }
}

impl AocDay {
    fn bad_bfs(&self, graph: &HashMap<Vec2D<usize>, HashMap<Vec2D<usize>, usize>>) -> usize {
        let mut path_length = 0;
        let mut to_visit = VecDeque::new();
        to_visit.push_front(vec![self.start]);
        while let Some(node) = to_visit.pop_front() {
            if node.last() == Some(self.end).as_ref() {
                let length = node.windows(2).fold(0, |acc, v| acc + graph[&v[0]][&v[1]]);
                path_length = path_length.max(length);
                continue;
            }
            for next_move in graph[node.last().unwrap()].keys() {
                if node.contains(next_move) {
                    continue;
                }
                let mut new = node.clone();
                new.push(*next_move);
                to_visit.push_back(new);
            }
        }
        path_length
    }

    fn get_longest(&self, start: &Vec2D<usize>, end: &Vec2D<usize>) -> Option<Vec<Vec2D<usize>>> {
        let mut path = None;
        let mut path_length = 0;
        let mut to_visit = VecDeque::new();
        to_visit.push_front(vec![*start]);
        while let Some(node) = to_visit.pop_front() {
            if node.last() == Some(end) {
                if node.len() > path_length {
                    path = Some(node.clone());
                    path_length = node.len();
                }
                continue;
            }
            for next_move in self.moves(node.last().unwrap(), false) {
                if node.contains(&next_move) || (next_move != *end && self.poi.contains(&next_move))
                {
                    continue;
                }
                let mut new = node.clone();
                new.push(next_move);
                to_visit.push_back(new);
            }
        }
        path
    }

    fn intersections(&self) -> Vec<Vec2D<usize>> {
        self.trails
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(c, tile)| {
                        if *tile == Tile::Path {
                            if self.moves(&Vec2D(r, c), false).len() > 2 {
                                Some(Vec2D(r, c))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn moves(&self, point: &Vec2D<usize>, check_slopes: bool) -> Vec<Vec2D<usize>> {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .iter()
            .filter_map(|d| {
                // Don't step off the grid.
                if (point.0 == 0 && *d == Dir::North)
                    || (point.1 == 0 && *d == Dir::West)
                    || (point.0 == self.height - 1 && *d == Dir::South)
                    || (point.1 == self.width - 1 && *d == Dir::East)
                {
                    return None;
                }
                let pos = d.delta(point);
                let tile = &self.trails[pos.0][pos.1];
                if (check_slopes
                    && matches!(
                        (tile, d),
                        (Tile::Path, _)
                            | (Tile::SlopeUp, Dir::North)
                            | (Tile::SlopeRight, Dir::East)
                            | (Tile::SlopeDown, Dir::South)
                            | (Tile::SlopeLeft, Dir::West)
                    ))
                    || (!check_slopes && *tile != Tile::Forest)
                {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
    }

    fn _dump(&self) {
        for line in self.trails.iter() {
            for ch in line.iter() {
                print!(
                    "{}",
                    match ch {
                        Tile::Path => '.',
                        Tile::Forest => '#',
                        Tile::SlopeUp => '^',
                        Tile::SlopeRight => '>',
                        Tile::SlopeLeft => '<',
                        Tile::SlopeDown => 'v',
                    }
                )
            }
            println!();
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Path, // .
    Forest,     // #
    SlopeUp,    // ^
    SlopeRight, // >
    SlopeLeft,  // <
    SlopeDown,  // v
}

impl From<&char> for Tile {
    fn from(value: &char) -> Self {
        match *value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::SlopeUp,
            '>' => Self::SlopeRight,
            '<' => Self::SlopeLeft,
            'v' => Self::SlopeDown,
            _ => unreachable!("Found unknown character"),
        }
    }
}
