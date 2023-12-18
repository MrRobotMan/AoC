use std::{collections::HashMap, fmt::Display};

use aoc::{
    runner::{output, run_solution, Runner},
    search::{dijkstra, Searcher, Weighted},
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
        output("Unsolved")
    }
}

impl AocDay {
    fn get_path(&self, limits: (usize, usize)) -> usize {
        let path = dijkstra(
            &Node {
                pos: (0, 0),
                dir: Dir::East,
                steps: 1,
                limits,
                target: (self.map.size.0 - 1, self.map.size.1 - 1),
            },
            &self.map,
        )
        .unwrap_or_default()
        .iter()
        .map(|n| n.pos)
        .collect::<Vec<_>>();
        path.iter()
            .skip(1)
            .fold(0, |acc, node| acc + self.map.grid[node])
    }
}

#[derive(Debug, Default)]
struct Map {
    grid: HashMap<(usize, usize), usize>,
    size: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    pos: (usize, usize),
    dir: Dir,
    steps: usize,
    limits: (usize, usize),
    target: (usize, usize),
}

impl Searcher for Node {
    fn moves(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let positions = vec![
            (self.pos.0.saturating_sub(1), self.pos.1),        // Up
            ((self.pos.0 + 1).min(self.target.0), self.pos.1), // Down
            (self.pos.0, (self.pos.1 + 1).min(self.target.1)), // Right
            (self.pos.0, self.pos.1.saturating_sub(1)),        // Left
        ];
        let mut steps = [1; 4];
        steps[self.dir as usize] = self.steps + 1;
        let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];
        let dont_go = match self.dir {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        };
        positions
            .into_iter()
            .zip(steps.into_iter().zip(dirs))
            .filter_map(|(pos, (steps, dir))| {
                if self.limits.0 <= steps
                    && steps <= self.limits.1
                    && pos != self.pos
                    && dir != dont_go
                {
                    let mut node = self.clone();
                    node.pos = pos;
                    node.dir = dir;
                    node.steps = steps;
                    Some(node)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_done(&self) -> bool {
        self.pos == self.target
    }
}

impl Weighted<Node> for Map {
    fn weight(&self, node: &Node) -> usize {
        self.grid[&node.pos]
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
}
