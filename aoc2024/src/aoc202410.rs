use std::collections::{HashMap, HashSet};

use aoc::{
    read_grid_numbers,
    runner::{output, Runner},
    Point, CARDINALS,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    map: HashMap<Point<i64>, u8>,
    zeroes: Vec<Point<i64>>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn paths(&self, loc: Point<i64>) -> usize {
        let mut queue = vec![loc];
        let mut visited = HashSet::new();
        let mut nines = HashSet::new();
        while let Some(loc) = queue.pop() {
            visited.insert(loc);
            for dir in CARDINALS {
                let point = loc + dir;
                if visited.contains(&point) {
                    continue;
                }
                if let Some(v) = self.map.get(&point) {
                    if v.saturating_sub(self.map[&loc]) == 1 {
                        if *v == 9 {
                            nines.insert(point);
                        } else {
                            queue.push(point);
                        }
                    }
                }
            }
        }
        nines.len()
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 10)
    }

    fn parse(&mut self) {
        for (r, row) in read_grid_numbers(&self.input).into_iter().enumerate() {
            for (c, value) in row.into_iter().enumerate() {
                self.map.insert(Point(r as i64, c as i64), value);
                if value == 0 {
                    self.zeroes.push(Point(r as i64, c as i64));
                }
            }
        }
    }

    fn part1(&mut self) -> String {
        output(
            self.zeroes
                .iter()
                .map(|loc| self.paths(*loc))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
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
    }
}
