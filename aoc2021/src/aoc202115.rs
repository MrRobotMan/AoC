use core::panic;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

use aoc::runner::{output, Runner};
use aoc::Vec2D;
use aoc::{read_grid_to_map, CARDINALS};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    cave: Cave,
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
        (2021, 15)
    }

    fn parse(&mut self) {
        self.cave = read_grid_to_map(&self.input).into();
    }

    fn part1(&mut self) -> String {
        output(self.cave.find_path_cost())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug, Default)]
struct Cave {
    map: HashMap<Vec2D<i64>, usize>,
    size: Vec2D<i64>,
}

impl Cave {
    fn find_path_cost(&self) -> usize {
        let Some((_, cost)) = dijkstra(
            &Vec2D(0, 0),
            |node| self.successors(node),
            |node| self.success(node),
        ) else {
            panic!("No path found")
        };
        cost
    }

    fn successors(&self, cur: &Vec2D<i64>) -> Vec<(Vec2D<i64>, usize)> {
        CARDINALS
            .iter()
            .filter_map(|d| {
                let next = d + cur;
                self.map.get(&next).map(|v| (next, *v))
            })
            .collect()
    }

    fn success(&self, node: &Vec2D<i64>) -> bool {
        *node == self.size
    }
}

impl From<Vec<((usize, usize), char)>> for Cave {
    fn from(map: Vec<((usize, usize), char)>) -> Self {
        let mut cave = Self::default();
        let mut rows = 0;
        let mut cols = 0;
        for (point, val) in map {
            rows = rows.max(point.0);
            cols = cols.max(point.1);
            cave.map.insert(point.into(), (val as u8 - b'0') as usize);
        }
        cave.size = Vec2D(rows as i64, cols as i64);
        cave
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let cave: Cave = read_grid_to_map(
            "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
        )
        .into();
        let expected = 40;
        let actual = cave.find_path_cost();
        assert_eq!(expected, actual);
    }
}
