use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    players: [(usize, usize); 2],
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
        (2021, 21)
    }

    fn parse(&mut self) {
        for (player, line) in read_lines(&self.input).iter().enumerate() {
            self.players[player] = (line.split_once(": ").unwrap().1.parse().unwrap(), 0);
        }
    }

    fn part1(&mut self) -> String {
        let mut players = self.players;
        let turns = play(&mut players, 1000);
        output(turns * 3 * players.iter().map(|p| p.1).min().unwrap())
    }

    fn part2(&mut self) -> String {
        let mut players = self.players;
        output(play_dirac(&mut players).iter().max().unwrap())
    }
}

fn play(players: &mut [(usize, usize); 2], max_score: usize) -> usize {
    let mut turns = 0;
    let mut pos = 0;
    while roll(players, turns % 2, &mut pos) < max_score {
        turns += 1;
    }
    turns + 1
}

fn play_dirac(players: &mut [(usize, usize); 2]) -> [usize; 2] {
    let mut player = 0;
    let mut complete = [0, 0];
    let mut active = HashMap::new();
    active.insert(*players, 1);
    while !active.is_empty() {
        // for _ in 0..3 {
        let mut temp = HashMap::new();
        for (state, cnt) in active {
            let rolls = [
                3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
            ];
            for roll in rolls {
                let mut s = state;
                let (pos, score) = &mut s[player];
                *pos = pos.increment(roll, 10);
                *score += *pos;
                if *score >= 21 {
                    complete[player] += cnt;
                } else {
                    let ent = temp.entry(s).or_default();
                    *ent += cnt;
                }
            }
        }
        player = player.increment(1, 1);
        active = temp;
        // println!("{active:?}\n");
    }
    complete
}
fn roll(players: &mut [(usize, usize); 2], player: usize, pos: &mut usize) -> usize {
    let steps = roll_die(pos);
    let (pos, score) = &mut players.get_mut(player).expect("Bad player index");
    *pos = pos.increment(steps, 10);
    *score += *pos;
    *score
}

fn roll_die(pos: &mut usize) -> usize {
    let mut value = 0;
    for _ in 1..=3 {
        *pos = pos.increment(1, 100);
        value += *pos % 10;
    }
    value % 10
}

trait Increment {
    fn increment(self, inc: Self, lim: Self) -> Self;
}

impl Increment for usize {
    fn increment(self, inc: Self, lim: Self) -> Self {
        let mut res = self;
        res += inc;
        if res > lim {
            res %= lim;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_first_four() {
        let mut players = [(4, 0), (8, 0)];

        let mut pos = 0;
        roll(&mut players, 0, &mut pos);
        assert_eq!(pos, 3);
        assert_eq!(&[(10, 10), (8, 0)], &players);
        roll(&mut players, 1, &mut pos);
        assert_eq!(&[(10, 10), (3, 3)], &players);
        roll(&mut players, 0, &mut pos);
        assert_eq!(&[(4, 14), (3, 3)], &players);
        roll(&mut players, 1, &mut pos);
        assert_eq!(&[(4, 14), (6, 9)], &players);
    }

    #[test]
    fn test_example1() {
        let mut players = [(4, 0), (8, 0)];
        let turns = play(&mut players, 1000);
        assert_eq!(&[(10, 1000), (3, 745)], &players);
        assert_eq!(331, turns);
        assert_eq!(
            739785,
            turns * 3 * players.iter().map(|p| p.1).min().unwrap()
        )
    }

    #[test]
    fn test_roll_dirac() {
        let mut players = [(4, 0), (8, 0)];
        let expected = [444356092776315, 341960390180808];
        let actual = play_dirac(&mut players);
        assert_eq!(expected, actual);
    }
}
