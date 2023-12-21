use std::collections::{HashMap, HashSet};

use aoc::{
    runner::{output, run_solution, Runner},
    Dir,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day21.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    garden: Garden,
    steps: usize,
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
            HashMap::from_iter(lines.into_iter().enumerate().flat_map(|(row, line)| {
                line.into_iter()
                    .enumerate()
                    .filter_map(|(col, ch)| {
                        if ch == '.' {
                            None
                        } else if ch == 'S' {
                            self.garden.start = (row as i64, col as i64);
                            None
                        } else {
                            Some(((row as i64, col as i64), ch))
                        }
                    })
                    .collect::<Vec<_>>()
            }));
    }

    fn part1(&mut self) -> Vec<String> {
        let mut visited = HashSet::new();
        visited.insert(self.garden.start);
        for _ in 0..self.steps {
            visited = visited
                .iter()
                .flat_map(|cell| self.garden.step(cell))
                .collect();
        }
        output(visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut visited = HashSet::new();
        visited.insert(self.garden.start);
        for s in 0..self.steps {
            println!("step {}", s + 1);
            visited = visited
                .iter()
                .flat_map(|cell| self.garden.step_infinite(cell))
                .collect();
        }
        output(visited.len())
    }
}

#[derive(Debug, Default, Clone)]
struct Garden {
    layout: HashMap<(i64, i64), char>,
    width: i64,
    height: i64,
    start: (i64, i64),
}

impl Garden {
    fn step(&self, position: &(i64, i64)) -> Vec<(i64, i64)> {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .iter()
            .filter_map(|dir| {
                let new_pos = dir.delta(position);
                if new_pos.0 < 0
                    || new_pos.0 >= self.height
                    || new_pos.1 < 0
                    || new_pos.1 >= self.width
                    || self.layout.contains_key(&new_pos)
                {
                    None
                } else {
                    Some(new_pos)
                }
            })
            .collect()
    }

    fn step_infinite(&self, position: &(i64, i64)) -> Vec<(i64, i64)> {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .iter()
            .filter_map(|dir| {
                let new_pos = dir.delta(position);
                let mut check_pos = (new_pos.0 % self.height, new_pos.1 % self.width);
                if check_pos.0 < 0 {
                    check_pos.0 += self.height;
                }
                if check_pos.1 < 0 {
                    check_pos.1 += self.width;
                }
                if check_pos.0 < 0
                    || check_pos.0 >= self.height
                    || check_pos.1 < 0
                    || check_pos.1 >= self.width
                    || self.layout.contains_key(&check_pos)
                {
                    None
                } else {
                    Some(new_pos)
                }
            })
            .collect()
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

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            steps: 50,
            ..Default::default()
        };
        day.parse();
        let expected = 1594;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
