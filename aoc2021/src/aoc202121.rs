use std::collections::HashSet;

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
        output("Unsolved")
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
    let mut active = HashSet::new();
    active.insert(*players);
    // while !active.is_empty() {
    for _ in 0..3 {
        let mut temp = HashSet::new();
        for state in active {
            for (end, score) in DIRAC_ROLLS[state[player].0 - 1] {
                let mut s = state;
                let (pos, init_score) = &mut s[player];
                *init_score += score;
                *pos = end;
                if *init_score >= 21 {
                    complete[player] += 1;
                } else {
                    temp.insert(s);
                }
            }
        }
        player = player.increment(1, 1);
        active = temp;
        println!("{active:?}\n");
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
        // *pos += 1;
        // if *pos > 100 {
        //     *pos %= 100;
        // }
        value += *pos % 10;
    }
    value % 10
}
// From starting position 0..10 the possible outcomes for 3 rolls;
// Ex: from pos 1 roll 1,1,1 earning 9 points and ending on spot 4
//                     1,1,2 earning 10 points and ending on spot 5
// Ex: from pos 5 roll 3,1,2 earning 18 (8 + 9 + 1) points and ending on spot 1
#[rustfmt::skip]
const DIRAC_ROLLS: [[(usize, usize); 27]; 10] = [
    [
        (4, 9), (5, 10), (6, 11), (5, 11), (6, 12), (7, 13), (6, 13), (7, 14), (8, 15),
        (5, 12), (6, 13), (7, 14), (6, 14), (7, 15), (8, 16), (7, 16), (8, 17), (9, 18),
        (6, 15), (7, 16), (8, 17), (7, 17), (8, 18), (9, 19), (8, 19), (9, 20), (10, 21),
    ],
    [
        (5, 12), (6, 13), (7, 14), (6, 14), (7, 15), (8, 16), (7, 16), (8, 17), (9, 18),
        (6, 15), (7, 16), (8, 17), (7, 17), (8, 18), (9, 19), (8, 19), (9, 20), (10, 21),
        (7, 18), (8, 19), (9, 20), (8, 20), (9, 21), (10, 22), (9, 22), (10, 23), (1, 14),
    ],
    [
        (6, 15), (7, 16), (8, 17), (7, 17), (8, 18), (9, 19), (8, 19), (9, 20), (10, 21),
        (7, 18), (8, 19), (9, 20), (8, 20), (9, 21), (10, 22), (9, 22), (10, 23), (1, 14),
        (8, 21), (9, 22), (10, 23), (9, 23), (10, 24), (1, 15), (10, 25), (1, 16), (2, 17),
    ],
    [
        (7, 18), (8, 19), (9, 20), (8, 20), (9, 21), (10, 22), (9, 22), (10, 23), (1, 14),
        (8, 21), (9, 22), (10, 23), (9, 23), (10, 24), (1, 15), (10, 25), (1, 16), (2, 17),
        (9, 24), (10, 25), (1, 16), (10, 26), (1, 17), (2, 18), (1, 18), (2, 19), (3, 20),
    ],
    [
        (8, 21), (9, 22), (10, 23), (9, 23), (10, 24), (1, 15), (10, 25), (1, 16), (2, 17),
        (9, 24), (10, 25), (1, 16), (10, 26), (1, 17), (2, 18), (1, 18), (2, 19), (3, 20),
        (10, 27), (1, 18), (2, 19), (1, 19), (2, 20), (3, 21), (2, 11), (3, 12), (4, 13),
    ],
    [
        (9, 24), (10, 25), (1, 16), (10, 26), (1, 17), (2, 18), (1, 18), (2, 19), (3, 20),
        (10, 27), (1, 18), (2, 19), (1, 19), (2, 20), (3, 21), (2, 11), (3, 12), (4, 13),
        (1, 20), (2, 21), (3, 22), (2, 12), (3, 13), (4, 14), (3, 14), (4, 15), (5, 16),
    ],
    [
        (10, 27), (1, 18), (2, 19), (1, 19), (2, 20), (3, 21), (2, 11), (3, 12), (4, 13),
        (1, 20), (2, 21), (3, 22), (2, 12), (3, 13), (4, 14), (3, 14), (4, 15), (5, 16),
        (2, 13), (3, 14), (4, 15), (3, 15), (4, 16), (5, 17), (4, 17), (5, 18), (6, 19),
    ],
    [
        (1, 20), (2, 21), (3, 22), (2, 12), (3, 13), (4, 14), (3, 14), (4, 15), (5, 16),
        (2, 13), (3, 14), (4, 15), (3, 15), (4, 16), (5, 17), (4, 17), (5, 18), (6, 19),
        (3, 6), (4, 7), (5, 8), (4, 8), (5, 9), (6, 10), (5, 10), (6, 11), (7, 12),
    ],
    [
        (2, 13), (3, 14), (4, 15), (3, 15), (4, 16), (5, 17), (4, 17), (5, 18), (6, 19),
        (3, 6), (4, 7), (5, 8), (4, 8), (5, 9), (6, 10), (5, 10), (6, 11), (7, 12),
        (4, 9), (5, 10), (6, 11), (5, 11), (6, 12), (7, 13), (6, 13), (7, 14), (8, 15),
    ],
    [
        (3, 6), (4, 7), (5, 8), (4, 8), (5, 9), (6, 10), (5, 10), (6, 11), (7, 12),
        (4, 9), (5, 10), (6, 11), (5, 11), (6, 12), (7, 13), (6, 13), (7, 14), (8, 15),
        (5, 12), (6, 13), (7, 14), (6, 14), (7, 15), (8, 16), (7, 16), (8, 17), (9, 18),
    ],
];

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
        let expected = [444356092776315, 34196039018080808];
        let actual = play_dirac(&mut players);
        assert_eq!(expected, actual);
    }
}
