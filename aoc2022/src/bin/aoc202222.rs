use std::collections::HashMap;

use aoc::{
    runner::{output, run_solution, Runner},
    Point,
};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day22.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    board: HashMap<Point<usize>, char>,
    instructions: Vec<Instruction>,
    height: usize,
    width: usize,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 22)
    }

    fn parse(&mut self) {
        let mut lines = aoc::read_lines(&self.input);
        let instructions = lines.pop().unwrap();
        self.board = lines
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(c, ch)| {
                        if ch == '.' || ch == '#' {
                            Some((Point(r, c), ch))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        self.height = lines.len();
        self.width = lines.iter().map(|l| l.len()).max().unwrap();

        let mut distance = 0;
        for ch in instructions.chars() {
            match ch.to_digit(10) {
                Some(v) => distance = distance * 10 + v as usize,
                None => {
                    let turn = match ch {
                        'R' => Turn::Right,
                        'L' => Turn::Left,
                        _ => panic!("Unknown direction {ch}"),
                    };
                    self.instructions.push(Instruction { distance, turn });
                    distance = 0;
                }
            }
        }

        if cfg!(test) {
            for r in 0..self.height {
                for c in 0..self.width {
                    print!(
                        "{}",
                        if let Some(ch) = self.board.get(&Point(r, c)) {
                            *ch
                        } else {
                            ' '
                        }
                    );
                }
                println!();
            }
            println!("{:?}", self.instructions);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug)]
struct Instruction {
    distance: usize,
    turn: Turn,
}

#[derive(Debug)]
enum Turn {
    Right,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 6032;
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
        let expected = 0;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
