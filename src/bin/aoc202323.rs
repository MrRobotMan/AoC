use std::collections::{HashMap, VecDeque};

use aoc::{
    runner::{output, run_solution, Runner},
    search::get_path,
    Dir, Point,
};

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

        #[cfg(test)]
        self._dump();
    }

    fn part1(&mut self) -> Vec<String> {
        let visited = vec![Point(self.start.0, self.start.1)];
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl AocDay {
    fn bad_bfs(&self) -> Option<Vec<Point<usize>>> {
        let mut path = HashMap::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_front(self.start);
        while let Some(node) = to_visit.pop_front() {
            if node == self.end {
                return Some(get_path(path, node, &self.start));
            }
            for next_move in self.moves(&node) {
                if path.contains_key(&next_move) {
                    continue;
                }
                to_visit.push_back(next_move);
                path.insert(next_move, node);
            }
        }
        None
    }

    fn moves(&self, point: &Point<usize>) -> Vec<Point<usize>> {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .iter()
            .filter_map(|d| {
                if (point.0 == 0 && *d == Dir::North)
                    || (point.1 == 0 && *d == Dir::West)
                    || (point.0 == self.height - 1 && *d == Dir::South)
                    || (point.1 == self.width - 1 && *d == Dir::East)
                {
                    return None;
                }
                let pos = d.delta(point);
                if matches!(
                    (self.trails[pos.0][pos.1], d),
                    (Tile::Path, _)
                        | (Tile::SlopeUp, Dir::North)
                        | (Tile::SlopeRight, Dir::East)
                        | (Tile::SlopeDown, Dir::South)
                        | (Tile::SlopeLeft, Dir::West)
                ) {
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
