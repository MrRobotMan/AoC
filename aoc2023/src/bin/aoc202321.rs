use std::collections::{HashMap, HashSet};

use aoc::{
    runner::{output, run_solution, Runner},
    search::{Graph, Searcher},
    Dir, Point,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/day21.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    garden: Garden,
    steps: usize,
    shortest: HashMap<Point<i64>, i64>,
    visited: HashSet<Point<i64>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 21)
    }

    fn parse(&mut self) {
        let lines = aoc::read_grid(&self.input);
        self.garden.width = lines[0].len() as i64;
        self.garden.height = lines[1].len() as i64;
        if self.steps == 0 {
            self.steps = 64;
        }
        self.garden.layout =
            HashSet::from_iter(lines.into_iter().enumerate().flat_map(|(row, line)| {
                line.into_iter()
                    .enumerate()
                    .filter_map(|(col, ch)| {
                        if ch == '.' {
                            None
                        } else if ch == 'S' {
                            self.garden.start = Point(row as i64, col as i64);
                            None
                        } else {
                            Some(Point(row as i64, col as i64))
                        }
                    })
                    .collect::<Vec<_>>()
            }));
    }

    fn part1(&mut self) -> Vec<String> {
        self.visited.insert(self.garden.start);
        for step in 0..self.steps as i64 {
            self.visited = self
                .visited
                .iter()
                .flat_map(|cell| self.garden.step(cell))
                .collect();
            for p in self.visited.iter() {
                self.shortest
                    .entry(*p)
                    .and_modify(|v| *v = (step + 1).min(*v))
                    .or_insert(step + 1);
            }
        }
        output(self.visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        for step in self.steps as i64..(3 * self.steps as i64) {
            self.visited = self
                .visited
                .iter()
                .flat_map(|cell| self.garden.step(cell))
                .collect();
            for p in self.visited.iter() {
                self.shortest
                    .entry(*p)
                    .and_modify(|v| *v = (step + 1).min(*v))
                    .or_insert(step + 1);
            }
        }
        let evens_out = self
            .shortest
            .values()
            .filter(|v| **v % 2 == 0 && **v > 65)
            .count() as i64;
        let odds_out = self
            .shortest
            .values()
            .filter(|v| **v % 2 == 1 && **v > 65)
            .count() as i64;
        let evens = self.shortest.values().filter(|v| **v % 2 == 0).count() as i64;
        let odds = self.shortest.values().filter(|v| **v % 2 == 1).count() as i64;
        let n = (26501365 - self.garden.start.0) / self.garden.height;
        let visited = (n + 1).pow(2) * odds + n.pow(2) * evens - (n + 1) * odds_out + n * evens_out;
        output(visited)
    }
}

impl Searcher<Garden> for Point<i64> {
    fn moves(&self, graph: &Garden) -> Vec<Self>
    where
        Self: Sized,
    {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .iter()
            .filter_map(|dir| {
                let new_pos = dir.delta(&Point(self.0, self.1));
                if new_pos.0 < 0
                    || new_pos.0 >= graph.height() as i64
                    || new_pos.1 < 0
                    || new_pos.1 >= graph.width() as i64
                    || graph.layout.contains(&new_pos)
                {
                    None
                } else {
                    Some(Self(new_pos.0, new_pos.1))
                }
            })
            .collect()
    }

    fn is_done(&self, graph: &Garden) -> bool {
        self == &graph.target
    }
}

#[derive(Debug, Default, Clone)]
struct Garden {
    layout: HashSet<Point<i64>>,
    width: i64,
    height: i64,
    start: Point<i64>,
    target: Point<i64>,
}

impl Graph for Garden {
    fn value(&self, _row: usize, _col: usize) -> usize {
        0
    }

    fn height(&self) -> usize {
        self.height as usize
    }

    fn width(&self) -> usize {
        self.width as usize
    }
}

impl Garden {
    fn step(&self, position: &Point<i64>) -> Vec<Point<i64>> {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .iter()
            .filter_map(|dir| {
                let new_pos = dir.delta(position);
                if new_pos.0 < 0
                    || new_pos.0 >= self.height
                    || new_pos.1 < 0
                    || new_pos.1 >= self.width
                    || self.layout.contains(&new_pos)
                {
                    None
                } else {
                    Some(new_pos)
                }
            })
            .collect()
    }

    fn _dump(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!(
                    "{}",
                    match self.layout.get(&Point(row, col)) {
                        Some(_) => '#',
                        None => '.',
                    }
                )
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            steps: 6,
            ..Default::default()
        };
        day.parse();
        let expected = 16;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
