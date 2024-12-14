use std::{collections::HashSet, str::FromStr};

use aoc::{
    measure::CARDINALS,
    read_lines,
    runner::{output, Runner},
    Vec2D,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    guards: Vec<Guard>,
    rows: i64,
    cols: i64,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            rows: 103,
            cols: 101,
            ..Default::default()
        }
    }
    fn show(&self, guards: &[Guard]) {
        let guards = guards.iter().map(|g| g.pos).collect::<HashSet<_>>();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if guards.contains(&Vec2D(col, row)) {
                    print!("#");
                } else {
                    print!(" ")
                }
            }
            println!();
        }
        println!()
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 14)
    }

    fn parse(&mut self) {
        let lines = read_lines(&self.input);
        self.guards = lines.iter().map(|line| line.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> String {
        let mut guards = self.guards.clone();
        for _ in 0..100 {
            for guard in guards.iter_mut() {
                guard.step(self.cols, self.rows);
            }
        }
        let top_left = guards
            .iter()
            .filter(|guard| guard.pos.0 < (self.cols / 2) && guard.pos.1 < (self.rows / 2))
            .count();
        let top_right = guards
            .iter()
            .filter(|guard| guard.pos.0 > (self.cols / 2) && guard.pos.1 < (self.rows / 2))
            .count();
        let bottom_left = guards
            .iter()
            .filter(|guard| guard.pos.0 < (self.cols / 2) && guard.pos.1 > (self.rows / 2))
            .count();
        let bottom_right = guards
            .iter()
            .filter(|guard| guard.pos.0 > (self.cols / 2) && guard.pos.1 > (self.rows / 2))
            .count();
        output(top_left * top_right * bottom_left * bottom_right)
    }

    fn part2(&mut self) -> String {
        let mut guards = self.guards.clone();
        for idx in 0..(self.rows * self.cols) {
            for guard in guards.iter_mut() {
                guard.step(self.cols, self.rows);
            }
            if is_tree(&guards) || idx == 8279 {
                self.show(&guards);
                return output(idx + 1);
            }
        }
        output("No tree")
    }
}

fn is_tree(guards: &[Guard]) -> bool {
    // Tree is made when there's a rectangle frame
    let guards = guards.iter().map(|g| g.pos).collect::<HashSet<_>>();
    for guard in &guards {
        if makes_rect(*guard, &guards) {
            return true;
        }
    }
    false
}

fn makes_rect(pos: Vec2D<i64>, guards: &HashSet<Vec2D<i64>>) -> bool {
    let start = pos;
    let mut visited = HashSet::new();
    let mut queue = vec![pos];
    while let Some(p) = queue.pop() {
        if visited.insert(p) {
            for dir in CARDINALS.iter() {
                let next = *dir + p;
                if next == start && visited.len() > 80 {
                    return true;
                }
                if guards.contains(&next) {
                    queue.push(next);
                }
            }
        }
    }
    false
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Guard {
    pos: Vec2D<i64>,
    vel: Vec2D<i64>,
}

impl Guard {
    fn step(&mut self, cols: i64, rows: i64) {
        self.pos += self.vel + Vec2D(cols, rows);
        self.pos.0 %= cols;
        self.pos.1 %= rows;
    }
}

impl FromStr for Guard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.trim().split_once(' ').unwrap();
        let pos = pos[2..]
            .split(',')
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec2D<i64>>();
        let vel = vel[2..]
            .split(',')
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec2D<i64>>();
        Ok(Self { pos, vel })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_guard_parse() {
        let expected = Ok(Guard {
            pos: Vec2D(0, 4),
            vel: Vec2D(3, -3),
        });
        let actual = "p=0,4 v=3,-3".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_guard_step() {
        let mut guard = Guard {
            pos: Vec2D(2, 4),
            vel: Vec2D(2, -3),
        };
        guard.step(11, 7);
        assert_eq!(Vec2D(4, 1), guard.pos);
        guard.step(11, 7);
        assert_eq!(Vec2D(6, 5), guard.pos);
        guard.step(11, 7);
        assert_eq!(Vec2D(8, 2), guard.pos);
        guard.step(11, 7);
        assert_eq!(Vec2D(10, 6), guard.pos);
        guard.step(11, 7);
        assert_eq!(Vec2D(1, 3), guard.pos);
    }

    #[test]
    fn test_part1() {
        let mut day = AocDay::new(
            "p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3",
        );
        day.rows = 7;
        day.cols = 11;
        day.parse();
        let expected = "12";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
