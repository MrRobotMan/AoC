use std::str::FromStr;

use aoc::{
    measure::Point,
    runner::{output, Runner},
    Dir,
};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub instructions: Vec<Instruction>,
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
        (2023, 18)
    }

    fn parse(&mut self) {
        self.instructions = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> String {
        let (nodes, border) = build_map(&self.instructions);
        output(get_area(&nodes, border))
    }

    fn part2(&mut self) -> String {
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
        let Point(dx, dy) = inst.direction.scale(inst.distance);
        cur = (cur.0 + dx, cur.1 + dy);
        border.push(cur);
    }
    (border, length)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub direction: Dir,
    pub distance: i64,
    pub color: String,
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
