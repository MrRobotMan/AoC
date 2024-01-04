use aoc::{
    runner::{output, run_solution, Runner},
    Dir, Point,
};
use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, hash::Hash};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day24.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    groves: Vec<HashMap<Point<i64>, Vec<Valley>>>,
    height: i64,
    width: i64,
    start: Point<i64>,
    end: Point<i64>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 24)
    }

    fn parse(&mut self) {
        self.groves.push(
            aoc::read_grid(&self.input)
                .iter()
                .enumerate()
                .flat_map(|(r, line)| {
                    self.height = self.height.max(r as i64 + 1);
                    line.iter()
                        .enumerate()
                        .filter_map(|(c, ch)| {
                            self.width = self.width.max(c as i64 + 1);
                            match ch {
                                '#' => Some((Point(r as i64, c as i64), vec![Valley::Wall])),
                                '^' => Some((
                                    Point(r as i64, c as i64),
                                    vec![Valley::Blizzard(Dir::North)],
                                )),
                                'v' => Some((
                                    Point(r as i64, c as i64),
                                    vec![Valley::Blizzard(Dir::South)],
                                )),
                                '<' => Some((
                                    Point(r as i64, c as i64),
                                    vec![Valley::Blizzard(Dir::West)],
                                )),
                                '>' => Some((
                                    Point(r as i64, c as i64),
                                    vec![Valley::Blizzard(Dir::East)],
                                )),
                                '.' => None,
                                _ => unreachable!("Unknown character {ch}"),
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        );
        for c in 0..self.width {
            if self.start == Point::default() && self.groves[0].get(&Point(0, c)).is_none() {
                self.start = Point(0, c);
            };
            if self.end == Point::default()
                && self.groves[0].get(&Point(self.height - 1, c)).is_none()
            {
                self.end = Point(self.height - 1, c);
            };
        }
        let mut next = Some(self.generate_maps());
        let base = Some(self.groves[0].clone());
        while next != base {
            self.groves.push(next.take().unwrap());
            next = Some(self.generate_maps());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let start = State {
            valley: 0,
            cur: self.start,
        };
        let path = dijkstra(
            &start,
            |state| self.step(state),
            |state| state.cur == self.end,
        )
        .unwrap();
        output(path.1)
    }

    fn part2(&mut self) -> Vec<String> {
        let there = dijkstra(
            &State {
                valley: 0,
                cur: self.start,
            },
            |state| self.step(state),
            |state| state.cur == self.end,
        )
        .unwrap();
        let back = dijkstra(
            there.0.last().unwrap(),
            |state| self.step(state),
            |state| state.cur == self.start,
        )
        .unwrap();
        let back_again = dijkstra(
            back.0.last().unwrap(),
            |state| self.step(state),
            |state| state.cur == self.end,
        )
        .unwrap();
        output(there.1 + back.1 + back_again.1)
    }
}

impl AocDay {
    fn step(&self, state: &State) -> Vec<(State, i64)> {
        let cur = state.cur;
        let valley = (state.valley + 1) % self.groves.len();
        let grove = &self.groves[valley];
        let mut res = Vec::new();
        if !grove.contains_key(&cur) {
            // Stand still
            res.push((State { cur, valley }, 1));
        }
        for d in Dir::iter() {
            let pos = cur + d.value();
            if (cur == self.start && d == Dir::North) || (cur == self.end && d == Dir::South) {
                continue;
            }
            if !grove.contains_key(&pos) {
                res.push((State { cur: pos, valley }, 1));
            }
        }
        res
    }

    fn generate_maps(&self) -> HashMap<Point<i64>, Vec<Valley>> {
        let cur = self.groves.last().unwrap();
        let mut valley = HashMap::new();
        for (point, vals) in cur {
            for val in vals {
                match (*point, val) {
                    (p, Valley::Wall) => {
                        valley.insert(p, vec![Valley::Wall]);
                    }
                    (p, Valley::Blizzard(d)) => {
                        let mut next = p + d.value();
                        // Hit a wall, wrap around.
                        if cur.get(&next) == Some(&vec![Valley::Wall]) {
                            match d {
                                Dir::North => next.0 = self.height - 2,
                                Dir::South => next.0 = 1,
                                Dir::East => next.1 = 1,
                                Dir::West => next.1 = self.width - 2,
                            }
                        }
                        valley
                            .entry(next)
                            .and_modify(|v| v.push(Valley::Blizzard(*d)))
                            .or_insert(vec![Valley::Blizzard(*d)]);
                    }
                };
            }
        }
        for vec in valley.values_mut() {
            vec.sort();
        }
        valley
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    valley: usize,
    cur: Point<i64>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Valley {
    Wall,
    Blizzard(Dir),
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 18;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 54;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
