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
    max: Game,
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
        output("Unsolved")
    }
}

fn valid_game(game: &Game, limit: &Game) -> bool {
    game.red <= limit.red && game.green <= limit.green && game.blue <= limit.blue
}

#[derive(Debug, Default)]
struct Game {
    red: u32,
    blue: u32,
    green: u32,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 8;
        let games = HashMap::from([
            (
                1,
                Game {
                    red: 4,
                    blue: 6,
                    green: 2,
                },
            ),
            (
                2,
                Game {
                    red: 1,
                    blue: 4,
                    green: 3,
                },
            ),
            (
                3,
                Game {
                    red: 20,
                    blue: 6,
                    green: 13,
                },
            ),
            (
                4,
                Game {
                    red: 14,
                    blue: 15,
                    green: 3,
                },
            ),
            (
                5,
                Game {
                    red: 6,
                    blue: 2,
                    green: 3,
                },
            ),
        ]);
        let limit = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let actual = games
            .iter()
            .filter_map(|(idx, game)| {
                if valid_game(game, &limit) {
                    Some(idx)
                } else {
                    None
                }
            })
            .sum();
        assert_eq!(expected, actual);
    }
}
