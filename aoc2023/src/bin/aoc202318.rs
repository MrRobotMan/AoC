use std::str::FromStr;

use aoc::{
    runner::{output, run_solution, Runner},
    Dir,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/day18.txt".into(),
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
        let (nodes, border) = build_map(&self.instructions);
        output(get_area(&nodes, border))
    }

    fn part2(&mut self) -> Vec<String> {
        let instructions = self
            .instructions
            .iter()
            .map(Instruction::flip)
            .collect::<Vec<_>>();
        let (nodes, border) = build_map(&instructions);
        output(get_area(&nodes, border))
    }
}

fn get_area(nodes: &[(i64, i64)], border: i64) -> i64 {
    let interior = ((nodes.windows(2).fold(0, |acc, nodes| {
        acc + nodes[0].0 * nodes[1].1 - nodes[0].1 * nodes[1].0
    }) + nodes.last().unwrap().0 * nodes[0].1
        - nodes.last().unwrap().1 * nodes[0].0)
        / 2)
    .abs(); // Shoelace area
    interior + border / 2 + 1 // Pick's solve for B*I
}

fn build_map(instructions: &[Instruction]) -> (Vec<(i64, i64)>, i64) {
    let mut cur = (0, 0);
    let mut length = 0;
    let mut border = Vec::new();
    for inst in instructions.iter() {
        length += inst.distance;
        let (dx, dy) = inst.direction.scale(inst.distance);
        cur = (cur.0 + dx, cur.1 + dy);
        border.push(cur);
    }
    (border, length)
}

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

    // #[test]
    // fn test_part2() {
    //     let mut day = AocDay {
    //         input: INPUT.into(),
    //         ..Default::default()
    // };
    //     day.parse();
    //     let expected = 952408144115_i64;
    //     let actual = day.part2()[0].parse().unwrap_or_default();
    //     assert_eq!(expected, actual);
    // }
}
