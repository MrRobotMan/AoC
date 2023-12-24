use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
    runner::{output, run_solution, Runner},
    Dir, Point,
};

use itertools::Itertools;
use pathfinding::directed::bfs::bfs;

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day23.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    trails: Vec<Vec<Tile>>,
    start: Point<usize>,
    end: Point<usize>,
    poi: HashSet<Point<usize>>,
    height: usize,
    width: usize,
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
        self.start = Point(
            0,
            self.trails[0]
                .iter()
                .position(|t| *t == Tile::Path)
                .unwrap(),
        );
        self.end = Point(
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

    fn part1(&mut self) -> Vec<String> {
        let mut path_lengths: HashMap<Point<usize>, HashMap<Point<usize>, usize>> = HashMap::new();
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
                    let end = *path.last().unwrap();
                    path_lengths
                        .entry(path[0])
                        .and_modify(|v| {
                            v.insert(end, path.len() - 1);
                        })
                        .or_insert(HashMap::from([(end, path.len() - 1)]));
                }
            };
        }
        output(self.bad_bfs(&path_lengths))
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl AocDay {
    fn bad_bfs(&self, graph: &HashMap<Point<usize>, HashMap<Point<usize>, usize>>) -> usize {
        let mut path_length = 0;
        let mut to_visit = VecDeque::new();
        to_visit.push_front(vec![self.start]);
        while let Some(node) = to_visit.pop_front() {
            if node.last() == Some(self.end).as_ref() {
                let length = node.windows(2).fold(0, |acc, v| acc + graph[&v[0]][&v[1]]);
                path_length = path_length.max(length);
                continue;
            }
            for next_move in graph[&node.last().unwrap()].keys() {
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

    fn intersections(&self) -> Vec<Point<usize>> {
        self.trails
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(c, tile)| {
                        if *tile == Tile::Path {
                            if self.moves(&Point(r, c), false).len() > 2 {
                                Some(Point(r, c))
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

    fn moves(&self, point: &Point<usize>, check_slopes: bool) -> Vec<Point<usize>> {
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
enum Tile {
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

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 94;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
