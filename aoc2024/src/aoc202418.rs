use aoc::{
    read_lines,
    runner::{output, Runner},
    Vec2D, CARDINALS,
};
use pathfinding::prelude::bfs;

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    grid: Vec<Vec2D<i64>>,
    size: Vec2D<i64>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            size: Vec2D(70, 70),
            ..Default::default()
        }
    }
    fn successors(&self, node: &Vec2D<i64>, byte_count: usize) -> Vec<Vec2D<i64>> {
        CARDINALS
            .iter()
            .filter_map(|dir| {
                let step = node + dir;
                if !self.grid[..byte_count].contains(&step)
                    && (0..=self.size.0).contains(&step.0)
                    && (0..=self.size.1).contains(&step.1)
                {
                    Some(step)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 18)
    }

    fn parse(&mut self) {
        self.grid = read_lines(&self.input)
            .iter()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Vec2D(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) -> String {
        let byte_count = match self.size {
            Vec2D(6, 6) => 12,
            _ => 1024,
        };
        output(
            bfs(
                &Vec2D(0, 0),
                |node| self.successors(node, byte_count),
                |node| *node == self.size,
            )
            .unwrap()
            .len()
                - 1,
        )
    }

    fn part2(&mut self) -> String {
        let mut idx = 0;
        while bfs(
            &Vec2D(0, 0),
            |node| self.successors(node, idx),
            |node| *node == self.size,
        )
        .is_some()
        {
            idx += 1;
        }
        let block = self.grid[idx - 1];
        output(format!("{},{}", block.0, block.1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            size: Vec2D(6, 6),
            input: "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
            .to_string(),
            ..Default::default()
        };
        day.parse();
        assert_eq!("22", day.part1());
        assert_eq!("6,1", day.part2());
    }
}
