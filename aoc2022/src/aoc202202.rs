use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    rounds: Vec<(Score, Score)>,
    known: Vec<(Score, Outcome)>,
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
        (2022, 2)
    }

    fn parse(&mut self) {
        for line in aoc::read_lines(&self.input) {
            self.rounds.push({
                let mut chars = line.chars();
                (
                    Score::from_code(chars.next().unwrap()),
                    Score::from_code(chars.last().unwrap()),
                )
            });
            self.known.push({
                let mut chars = line.chars();
                (
                    Score::from_code(chars.next().unwrap()),
                    Outcome::from_code(chars.last().unwrap()),
                )
            });
        }
    }

    fn part1(&mut self) -> String {
        output(
            self.rounds
                .iter()
                .fold(0, |acc, (opp, you)| acc + you.value() + you.battle(*opp)),
        )
    }

    fn part2(&mut self) -> String {
        output(self.known.iter().fold(0, |acc, (opp, you)| {
            let you = you.outcome(opp);
            acc + you.value() + you.battle(*opp)
        }))
    }
}

#[derive(Clone, Copy)]
enum Score {
    Rock,
    Paper,
    Scissors,
}

impl Score {
    fn from_code(code: char) -> Self {
        match code {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid code"),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn battle(&self, player_two: Score) -> u32 {
        match (self, player_two) {
            (Self::Rock, Self::Rock) => 3,
            (Self::Rock, Self::Paper) => 0,
            (Self::Rock, Self::Scissors) => 6,
            (Self::Paper, Self::Rock) => 6,
            (Self::Paper, Self::Paper) => 3,
            (Self::Paper, Self::Scissors) => 0,
            (Self::Scissors, Self::Rock) => 0,
            (Self::Scissors, Self::Paper) => 6,
            (Self::Scissors, Self::Scissors) => 3,
        }
    }

    fn beats(val: &Self) -> Self {
        match val {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn loses(val: &Self) -> Self {
        match val {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_code(code: char) -> Self {
        match code {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid code"),
        }
    }

    fn outcome(&self, player_two: &Score) -> Score {
        match self {
            Self::Draw => *player_two,
            Self::Win => Score::beats(player_two),
            Self::Lose => Score::loses(player_two),
        }
    }
}
