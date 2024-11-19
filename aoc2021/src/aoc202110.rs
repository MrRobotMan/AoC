use aoc::{
    read_lines,
    runner::{output, Runner},
};

const OPENING: [char; 4] = ['(', '{', '[', '<'];
const CLOSING: [char; 4] = [')', '}', ']', '>'];

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    subsystem_lines: Vec<(Vec<char>, State)>,
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
        (2021, 10)
    }

    fn parse(&mut self) {
        self.subsystem_lines = read_lines(&self.input)
            .iter_mut()
            .map(|l| {
                let chrs = l.chars().collect::<Vec<char>>();
                let state = (&chrs).into();
                (chrs, state)
            })
            .collect();
    }

    fn part1(&mut self) -> String {
        output(
            self.subsystem_lines
                .iter()
                .filter_map(|(_, s)| match s {
                    State::Error(ch) => Some(ch),
                    _ => None,
                })
                .map(score)
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn score(ch: &char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("Unknown bracket {ch}"),
    }
}

enum State {
    Complete,
    Incomplete(Vec<char>),
    Error(char),
}

impl From<&Vec<char>> for State {
    fn from(value: &Vec<char>) -> Self {
        let mut closing = vec![];
        for chr in value {
            if let Some(idx) = OPENING.iter().position(|c| c == chr) {
                closing.push(CLOSING[idx]);
            } else if closing.pop() != Some(*chr) {
                return Self::Error(*chr);
            }
        }
        if closing.is_empty() {
            Self::Complete
        } else {
            Self::Incomplete(closing)
        }
    }
}
