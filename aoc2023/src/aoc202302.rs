use core::panic;
use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    input: String,
    games: HashMap<u32, Game>,
    max: Game,
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
        (2023, 2)
    }

    fn parse(&mut self) {
        self.max = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        for line in read_lines(&self.input) {
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
        output(
            self.games
                .iter()
                .filter_map(|(game_num, pulls)| {
                    if valid_game(pulls, &self.max) {
                        Some(game_num)
                    } else {
                        None
                    }
                })
                .sum::<u32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output(
            self.games
                .values()
                .map(|game| game.red * game.blue * game.green)
                .sum::<u32>(),
        )
    }
}

pub fn valid_game(game: &Game, limit: &Game) -> bool {
    game.red <= limit.red && game.green <= limit.green && game.blue <= limit.blue
}

#[derive(Debug, Default)]
pub struct Game {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}
