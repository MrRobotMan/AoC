use std::collections::{HashMap, HashSet};

use aoc::{
    read_grid,
    runner::{output, Runner},
    Vec2D,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    antennae: HashMap<char, Vec<Vec2D<i64>>>,
    max: (i64, i64),
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
        (2024, 8)
    }

    fn parse(&mut self) {
        read_grid(&self.input)
            .iter()
            .enumerate()
            .for_each(|(r, row)| {
                row.iter().enumerate().for_each(|(c, ch)| {
                    if *ch != '.' {
                        self.antennae
                            .entry(*ch)
                            .and_modify(|v| v.push(Vec2D(r as i64, c as i64)))
                            .or_insert_with(|| vec![Vec2D(r as i64, c as i64)]);
                    }
                    self.max.1 = self.max.1.max(c as i64);
                });
                self.max.0 = self.max.0.max(r as i64);
            });
    }

    fn part1(&mut self) -> String {
        let mut antinodes = HashSet::new();
        self.antennae.values().for_each(|arr| {
            arr.iter().enumerate().for_each(|(idx, left)| {
                arr[idx + 1..].iter().for_each(|right| {
                    get_antinodes(*left, *right).iter().for_each(|node| {
                        if node.0 >= 0
                            && node.0 <= self.max.0
                            && node.1 >= 0
                            && node.1 <= self.max.1
                        {
                            antinodes.insert(*node);
                        }
                    });
                })
            })
        });
        output(antinodes.len())
    }

    fn part2(&mut self) -> String {
        let mut antinodes = HashSet::new();
        self.antennae.values().for_each(|arr| {
            arr.iter().enumerate().for_each(|(idx, left)| {
                arr[idx + 1..].iter().for_each(|right| {
                    self.get_all_antinodes(*left, *right)
                        .iter()
                        .for_each(|node| {
                            antinodes.insert(*node);
                        });
                })
            })
        });
        output(antinodes.len())
    }
}

impl AocDay {
    fn get_all_antinodes(&self, a: Vec2D<i64>, b: Vec2D<i64>) -> HashSet<Vec2D<i64>> {
        let delta = b - a; // Point(row, col) => (dy, dx)
        if delta.1 == 0 {
            panic!("Slope vertical! {a:?}, {b:?}") // Let's see if we have vertical lines.
        }
        let slope = delta.0 as f64 / delta.1 as f64;
        let mut res = HashSet::new();
        for col in 0..=self.max.1 {
            // Max stored as (rows, cols)...max.1 = cols = x
            let row = slope * ((col - a.1) as f64) + a.0 as f64; // (y-y1) = m*(x-x1) => y = m*(x-x1) + y1
            if row.fract() == 0.0 && row as i64 <= self.max.0 && row as i64 >= 0 {
                // Row is an integer and in bounds
                res.insert(Vec2D(row as i64, col));
            }
        }
        res
    }
}
fn get_antinodes(a: Vec2D<i64>, b: Vec2D<i64>) -> [Vec2D<i64>; 2] {
    let delta = b - a;
    [a - delta, b + delta]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_antinodes1() {
        let expected = [Vec2D(1, 1), Vec2D(7, 7)];
        let actual = get_antinodes(Vec2D(3, 3), Vec2D(5, 5));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinodes2() {
        let expected = [Vec2D(-1, -2), Vec2D(5, 7)];
        let actual = get_antinodes(Vec2D(1, 1), Vec2D(3, 4));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinodes3() {
        let expected = [Vec2D(6, 2), Vec2D(0, 8)];
        let actual = get_antinodes(Vec2D(4, 4), Vec2D(2, 6));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinodes4() {
        let expected = [Vec2D(0, 6), Vec2D(6, 3)];
        let actual = get_antinodes(Vec2D(2, 5), Vec2D(4, 4));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ex2() {
        let mut day = AocDay::new(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        );
        day.parse();
        assert_eq!("34", day.part2());
    }
}
