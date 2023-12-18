use std::str::FromStr;

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
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    direction: Dir,
    distance: usize,
    color: String,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let direction = parts.next().unwrap().parse()?;
        let distance = parts.next().unwrap().parse::<usize>().unwrap();
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
}
