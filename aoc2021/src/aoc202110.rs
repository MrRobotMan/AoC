use std::collections::HashMap;

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
                    State::Error(ch) => Some(error_score(ch)),
                    _ => None,
                })
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        let mut scores = self
            .subsystem_lines
            .iter()
            .filter_map(|(_, s)| match s {
                State::Incomplete(chrs) => Some(correction_score(chrs)),
                _ => None,
            })
            .collect::<Vec<_>>();
        scores.sort();
        output(scores[scores.len() / 2])
    }
}

fn error_score(ch: &char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("Unknown bracket {ch}"),
    }
}

fn correction_score(ch: &[char]) -> usize {
    let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    ch.iter().fold(0, |acc, v| acc * 5 + scores[v])
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
            closing.reverse();
            Self::Incomplete(closing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_correction() {
        let state = std::convert::Into::<State>::into(
            &"<{([{{}}[<[[[<>{}]]]>[]]".chars().collect::<Vec<_>>(),
        );
        if let State::Incomplete(actual) = state {
            assert_eq!(vec![']', ')', '}', '>'], actual);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_calc_correction() {
        let correction = vec![']', ')', '}', '>'];
        assert_eq!(294, correction_score(&correction))
    }

    #[test]
    fn test_example() {
        let mut day = AocDay::new("[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]");
        day.parse();
        for (line, res) in &day.subsystem_lines {
            if let State::Incomplete(add) = res {
                println!(
                    "{} -> {} = {}",
                    line.iter().collect::<String>(),
                    add.iter().collect::<String>(),
                    correction_score(add)
                );
            }
        }
        assert_eq!("288957", day.part2())
    }
}
