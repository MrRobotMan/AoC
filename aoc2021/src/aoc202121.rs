use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    players: [(usize, usize); 2],
    die_position: usize,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn play(&mut self, max_score: usize) -> usize {
        let mut turns = 1;
        while self.roll((turns - 1) % 2) < max_score {
            turns += 1;
        }
        turns
    }

    fn roll(&mut self, player: usize) -> usize {
        let steps = self.roll_die();
        let (pos, score) = &mut self.players.get_mut(player).expect("Bad player index");
        *pos += steps;
        if *pos > 10 {
            *pos %= 10;
        }
        *score += *pos;
        *score
    }

    fn roll_die(&mut self) -> usize {
        let mut value = 0;
        for _ in 1..=3 {
            self.die_position += 1;
            if self.die_position > 100 {
                self.die_position %= 100;
            }
            value += self.die_position % 10;
        }
        value % 10
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
        let turns = self.play(1000);
        output(turns * 3 * self.players.iter().map(|p| p.1).min().unwrap())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_first_four() {
        let mut day = AocDay {
            input: "".into(),
            players: [(4, 0), (8, 0)],
            die_position: 0,
        };
        day.roll(0);
        assert_eq!(day.die_position, 3);
        assert_eq!(&[(10, 10), (8, 0)], &day.players);
        day.roll(1);
        assert_eq!(&[(10, 10), (3, 3)], &day.players);
        day.roll(0);
        assert_eq!(&[(4, 14), (3, 3)], &day.players);
        day.roll(1);
        assert_eq!(&[(4, 14), (6, 9)], &day.players);
    }

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            input: "".into(),
            players: [(4, 0), (8, 0)],
            die_position: 0,
        };
        let turns = day.play(1000);
        assert_eq!(&[(10, 1000), (3, 745)], &day.players);
        assert_eq!(331, turns);
        assert_eq!(
            739785,
            turns * 3 * day.players.iter().map(|p| p.1).min().unwrap()
        )
    }
}
