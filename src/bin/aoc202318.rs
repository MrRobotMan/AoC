use std::{collections::HashMap, str::FromStr};

use aoc::{
    runner::{output, run_solution, Runner},
    Dir,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day18.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    instructions: Vec<Instruction>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 18)
    }

    fn parse(&mut self) {
        self.instructions = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let (dims, border) = build_map(&self.instructions);
        let vol = get_volume(&dims, &border);
        output(vol)
    }

    fn part2(&mut self) -> Vec<String> {
        let instructions = self
            .instructions
            .iter()
            .map(Instruction::flip)
            .collect::<Vec<_>>();
        let (dims, border) = build_map(&instructions);
        let vol = get_volume(&dims, &border);
        output(vol)
    }
}

fn get_volume(dimensions: &Dimensions, border: &HashMap<(i64, i64), char>) -> i64 {
    let mut vol = 0;
    let mut inside = false;
    let mut cur = '.';
    for row in dimensions.0..=dimensions.1 {
        for col in dimensions.2..=dimensions.3 + 1 {
            match border.get(&(row, col)) {
                Some('|') => {
                    vol += 1;
                    inside = !inside;
                }
                Some('F') => {
                    vol += 1;
                    inside = !inside;
                    cur = 'F';
                }
                Some('L') => {
                    vol += 1;
                    inside = !inside;
                    cur = 'L';
                }
                Some('J') => {
                    vol += 1;
                    if cur == 'L' {
                        inside = !inside;
                        cur = '.';
                    }
                }
                Some('7') => {
                    vol += 1;
                    if cur == 'F' {
                        inside = !inside;
                        cur = '.';
                    }
                }
                Some('-') => vol += 1,
                None => {
                    if inside {
                        vol += 1;
                    }
                }
                c => panic!("Unknown option {:?}", c),
            }
        }
    }
    vol
}

fn build_map(instructions: &[Instruction]) -> (Dimensions, HashMap<(i64, i64), char>) {
    let mut cur = (0, 0);
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;
    let mut prev = instructions[0].direction;
    let mut border = HashMap::new();
    border.insert(
        (cur.0 as i64, cur.1 as i64),
        border_symbol(instructions.last().unwrap().direction, prev),
    );
    for instruction in instructions {
        border.insert(
            (cur.0 as i64, cur.1 as i64),
            border_symbol(prev, instruction.direction),
        );
        prev = instruction.direction;
        for _ in 0..instruction.distance {
            cur = instruction.direction.delta(&cur);
            min_row = min_row.min(cur.0 as i64);
            max_row = max_row.max(cur.0 as i64);
            min_col = min_col.min(cur.1 as i64);
            max_col = max_col.max(cur.1 as i64);
            border.insert(
                (cur.0 as i64, cur.1 as i64),
                border_symbol(prev, instruction.direction),
            );
        }
    }
    if cur == (0, 0) {
        border.insert(
            (cur.0 as i64, cur.1 as i64),
            border_symbol(prev, instructions[0].direction),
        );
    }
    ((min_row, max_row, min_col, max_col), border)
}

fn border_symbol(previous: Dir, current: Dir) -> char {
    match (previous, current) {
        (Dir::North, Dir::East) | (Dir::West, Dir::South) => 'F',
        (Dir::North, Dir::West) | (Dir::East, Dir::South) => '7',
        (Dir::South, Dir::East) | (Dir::West, Dir::North) => 'L',
        (Dir::South, Dir::West) | (Dir::East, Dir::North) => 'J',
        (Dir::North, Dir::North) | (Dir::South, Dir::South) => '|',
        (Dir::East, Dir::East) | (Dir::West, Dir::West) => '-',
        _ => panic!("Can't convert {previous:?} to {current:?}"),
    }
}

type Dimensions = (i64, i64, i64, i64);

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    direction: Dir,
    distance: i64,
    color: String,
}

impl Instruction {
    fn flip(&self) -> Self {
        let mut c = self
            .color
            .trim_start_matches("(#")
            .trim_end_matches(')')
            .to_string();
        let direction = match c.pop() {
            Some('0') => Dir::East,
            Some('1') => Dir::South,
            Some('2') => Dir::West,
            Some('3') => Dir::North,
            Some(c) => panic!("Unknown Direction {}", c),
            None => panic!("Can't convert color to number."),
        };
        let distance =
            i64::from_str_radix(&c, 16).unwrap_or_else(|_| panic!("Can't convert to distance."));
        Self {
            direction,
            distance,
            color: self.color.clone(),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let direction = parts.next().unwrap().parse()?;
        let distance = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap().into();
        Ok(Self {
            direction,
            distance,
            color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_parse() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 38;
        let actual = day
            .instructions
            .iter()
            .fold(0, |acc, inst| acc + inst.distance);
        assert_eq!(expected, actual);
        assert_eq!(
            Instruction {
                direction: Dir::East,
                distance: 6,
                color: "(#70c710)".into()
            },
            day.instructions[0]
        );
        assert_eq!(
            &Instruction {
                direction: Dir::North,
                distance: 2,
                color: "(#7a21e3)".into()
            },
            day.instructions.last().unwrap()
        );
    }

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 62;
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
        let expected = 952408144115_i64;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
