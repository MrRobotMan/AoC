use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc::{
    runner::{output, run_solution, Runner},
    search::{Searcher, Weighted},
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
        dijkstra(
            &Node {
                pos: (0, 0),
                dir: Dir::East,
                steps: 0,
                target: (self.map.size.0 - 1, self.map.size.1 - 1),
            },
            &self.map,
            limits,
        )
        .unwrap_or_default()
    }
}

fn dijkstra<U: Weighted<Node>>(
    start: &Node,
    map: &U,
    step_limits: (usize, usize),
) -> Option<usize> {
    let mut queue: HashSet<(Node, usize)> = HashSet::new();
    let mut dist: HashMap<(Node, usize), usize> = HashMap::new();
    let mut index: HashSet<(Node, usize)> = HashSet::new();
    let mut target = None;

    index.insert((start.clone(), 1));
    queue.insert((start.clone(), 1));
    dist.insert((start.clone(), 1), 0);

    while !queue.is_empty() {
        let shortest = queue
            .iter()
            .map(|item| (item, dist.get(item).unwrap()))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0
            .clone();

        if shortest.0.is_done() && shortest.1 >= step_limits.0 {
            // Found target. Let's build the path.
            target = Some(shortest);
            break;
        }

        if !queue.remove(&shortest) {
            panic!("Tried to remove shortest from queue but it was not found.")
        }

        for next_move in shortest.0.moves() {
            let step_count = if next_move.dir == shortest.0.dir {
                if shortest.1 >= step_limits.1 {
                    continue;
                }
                shortest.1 + 1
            } else {
                if shortest.1 < step_limits.0 {
                    continue;
                }
                1
            };
            let step = if queue.contains(&(next_move.clone(), step_count)) {
                next_move
            } else if index.insert((next_move.clone(), step_count)) {
                dist.insert((next_move.clone(), step_count), usize::MAX);
                queue.insert((next_move.clone(), step_count));
                next_move
            } else {
                continue;
            };
            let alt = dist[&shortest] + map.weight(&step);
            if alt < dist[&(step.clone(), step_count)] {
                dist.insert((step.clone(), step_count), alt);
            }
        }
    }
    target.map(|t| dist[&t])
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
                if pos != self.pos // Don't step off the grid.
                    && dir != dont_go
                // Don't go back the way you came.
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
