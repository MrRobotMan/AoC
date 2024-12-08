use std::collections::{HashMap, HashSet};

use aoc::{
    read_grid,
    runner::{output, Runner},
    Point,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    antennae: HashMap<char, Vec<Point<i64>>>,
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
                            .and_modify(|v| v.push(Point(r as i64, c as i64)))
                            .or_insert_with(|| vec![Point(r as i64, c as i64)]);
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
        output("Unsolved")
    }
}

fn get_antinodes(a: Point<i64>, b: Point<i64>) -> [Point<i64>; 2] {
    let delta = b - a;
    [a - delta, b + delta]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_antinodes1() {
        let expected = [Point(1, 1), Point(7, 7)];
        let actual = get_antinodes(Point(3, 3), Point(5, 5));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinodes2() {
        let expected = [Point(-1, -2), Point(5, 7)];
        let actual = get_antinodes(Point(1, 1), Point(3, 4));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinodes3() {
        let expected = [Point(6, 2), Point(0, 8)];
        let actual = get_antinodes(Point(4, 4), Point(2, 6));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinodes4() {
        let expected = [Point(0, 6), Point(6, 3)];
        let actual = get_antinodes(Point(2, 5), Point(4, 4));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinodes5() {}

    #[test]
    fn test_parse() {
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
        let expected = HashMap::from([
            (
                '0',
                vec![Point(1, 8), Point(2, 5), Point(3, 7), Point(4, 4)],
            ),
            ('A', vec![Point(5, 6), Point(8, 8), Point(9, 9)]),
        ]);
        assert_eq!(expected, day.antennae);
        assert_eq!("14", day.part1());
    }
}
