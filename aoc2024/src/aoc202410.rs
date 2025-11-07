use std::collections::{HashMap, HashSet};

use aoc::{
    CARDINALS, Vec2D, read_grid_numbers,
    runner::{Runner, output},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    map: HashMap<Vec2D<i64>, u8>,
    zeroes: Vec<Vec2D<i64>>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn path_counts(&self, loc: Vec2D<i64>, count_unique: bool) -> usize {
        let mut queue = vec![(loc, 0)];
        let mut visited = HashSet::new();
        let mut nines = 0;
        while let Some((loc, elev)) = queue.pop() {
            if count_unique || visited.insert(loc) {
                if elev == 9 {
                    nines += 1;
                    continue;
                }
                for dir in CARDINALS {
                    let point = loc + dir;
                    if let Some(v) = self.map.get(&point)
                        && elev + 1 == *v
                    {
                        queue.push((point, *v));
                    }
                }
            }
        }
        nines
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 10)
    }

    fn parse(&mut self) {
        for (r, row) in read_grid_numbers(&self.input).into_iter().enumerate() {
            for (c, value) in row.into_iter().enumerate() {
                self.map.insert(Vec2D(r as i64, c as i64), value);
                if value == 0 {
                    self.zeroes.push(Vec2D(r as i64, c as i64));
                }
            }
        }
    }

    fn part1(&mut self) -> String {
        output(
            self.zeroes
                .iter()
                .map(|loc| self.path_counts(*loc, false))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        output(
            self.zeroes
                .iter()
                .map(|loc| self.path_counts(*loc, true))
                .sum::<usize>(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        day.parse();
        assert_eq!("36", day.part1());
        assert_eq!("81", day.part2());
    }
}
