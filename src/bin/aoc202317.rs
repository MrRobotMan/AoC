use std::{collections::HashMap, fmt::Display};

use aoc::{
    runner::{output, run_solution, Runner},
    search::{a_star, Graph, Searcher, Weighted},
    Dir,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day17.txt".into(),
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
        output(self.get_path((1, 3)))
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.get_path((4, 10)))
    }
}

impl AocDay {
    fn get_path(&self, limits: (usize, usize)) -> usize {
        if let Some((path, score)) = a_star(
            &Node {
                pos: (0, 0),
                dir: Dir::East,
                steps: 0,
                limits,
            },
            &self.map,
            |node| (self.map.height() - node.pos.0) + (self.map.width() - node.pos.1),
        ) {
            for node in path {
                println!("{:?}", node.pos);
            }
            score
        } else {
            0
        }
    }
}

/*
2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>
*/

#[derive(Debug, Default)]
struct Map {
    grid: HashMap<(usize, usize), usize>,
    size: (usize, usize),
}

impl Graph for Map {
    fn value(&self, row: usize, col: usize) -> usize {
        self.grid[&(row, col)]
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
    pos: (usize, usize),
    dir: Dir,
    steps: usize,
    limits: (usize, usize),
}

impl Searcher<Map> for Node {
    fn moves(&self, map: &Map) -> Vec<Self>
    where
        Self: Sized,
    {
        // If we would step off the grid, set to the same spot.
        let positions = vec![
            (self.pos.0.saturating_sub(1), self.pos.1), // North
            ((self.pos.0 + 1).min(map.height() - 1), self.pos.1), // South
            (self.pos.0, (self.pos.1 + 1).min(map.width() - 1)), // East
            (self.pos.0, self.pos.1.saturating_sub(1)), // West
        ];
        let mut steps = [1; 4];
        steps[self.dir as usize] = self.steps + 1;
        let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];
        let dont_go = match self.dir {
            // Don't go back the way you came.
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        };
        let mut neighbors = Vec::new();
        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            // Don't back track
            if dir == dont_go {
                continue;
            }

            // Dont' go off the map
            if self.pos.0 == 0
                || self.pos.1 == 0
                || self.pos.0 == map.height() - 1
                || self.pos.1 == map.width() - 1
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
        positions
            .into_iter()
            .zip(steps.into_iter().zip(dirs))
            .filter_map(|(pos, (steps, dir))| {
                if pos == self.pos || dir == dont_go {
                    // Don't step off the grid, don't go back the way you came.
                    None
                } else if (dir == self.dir && steps <= self.limits.1)
                    || (dir != self.dir && steps >= self.limits.0)
                    || self.steps == 1
                {
                    Some(Self {
                        pos,
                        dir,
                        steps,
                        limits: self.limits,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_done(&self, map: &Map) -> bool {
        let target = (map.height() - 1, map.width() - 1);
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
        let size = (value.len(), value[0].len());
        let grid = HashMap::from_iter(value.into_iter().enumerate().flat_map(|(row, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col, chr)| ((row, col), (chr as u8 - b'0') as usize))
                .collect::<Vec<_>>()
        }));
        Self { grid, size }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                write!(f, "{}", self.grid[&(row, col)])?;
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
        let actual = day.get_path((1, 3));
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
        let actual = day.get_path((4, 10));
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
        let actual = day.get_path((4, 10));
        assert_eq!(expected, actual);
    }
}
