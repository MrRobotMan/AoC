use core::panic;
use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, run_solution, Runner},
};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    games: HashMap<u32, Game>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 2)
    }

    fn parse(&mut self) {
        for line in read_lines("inputs/2023/day02.txt") {
            if let Some((game, pulls)) = line.split_once(':') {
                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;
                let game = game[5..]
                    .parse::<_>()
                    .unwrap_or_else(|_| panic!("Can't parse {}", &game[5..]));
                for pull in pulls.trim().split(';') {
                    for ball in pull.trim().split(',') {
                        let (count, color) = ball
                            .trim()
                            .split_once(' ')
                            .unwrap_or_else(|| panic!("Can't split {ball}"));
                        let count = count
                            .parse::<_>()
                            .unwrap_or_else(|_| panic!("Can't convert {count} to count."));
                        match color {
                            "red" => red = red.max(count),
                            "blue" => blue = blue.max(count),
                            "green" => green = green.max(count),
                            _ => panic!("Unknown color {color}"),
                        };
                    }
                }
                self.games.insert(game, Game { red, blue, green });
            };
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, Default)]
struct Game {
    red: u32,
    blue: u32,
    green: u32,
}
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_game() {
        let expected = 0;
        let actual = 0;
        assert_eq!(expected, actual);
    }
}
