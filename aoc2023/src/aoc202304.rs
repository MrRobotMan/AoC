use std::collections::{HashMap, VecDeque};

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub cards: Vec<Card>,
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
        (2023, 4)
    }

    fn parse(&mut self) {
        self.cards = read_lines(&self.input)
            .iter()
            .map(|c| c.into())
            .collect::<Vec<Card>>();
    }

    fn part1(&mut self) -> String {
        output(self.cards.iter().map(Card::score).sum::<i64>())
    }

    fn part2(&mut self) -> String {
        output(self.total_cards())
    }
}

impl AocDay {
    pub fn total_cards(&self) -> usize {
        let mut remaining = VecDeque::from((0..self.cards.len()).collect::<Vec<_>>());
        let mut counter = HashMap::new();
        for c in 0..self.cards.len() {
            counter.insert(c, 1);
        }
        let matches = self
            .cards
            .iter()
            .map(|c| c.matches())
            .collect::<Vec<usize>>();
        while let Some(card) = remaining.pop_front() {
            for idx in 1..=matches[card] {
                *counter.entry(card + idx).or_default() += 1;
                remaining.push_back(card + idx);
            }
        }
        counter.values().sum()
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub winners: Vec<i64>,
    pub plays: Vec<i64>,
}

impl Card {
    pub fn score(&self) -> i64 {
        let mut score = 0;
        for play in &self.plays {
            if self.winners.contains(play) {
                score = 1.max(score * 2);
            }
        }
        score
    }

    fn matches(&self) -> usize {
        self.plays
            .iter()
            .filter(|p| self.winners.contains(p))
            .count()
    }
}

impl<T: AsRef<str>> From<T> for Card {
    fn from(value: T) -> Self {
        let (_, tail) = value.as_ref().split_once(": ").unwrap();
        let (winners, plays) = tail.split_once(" | ").unwrap();
        Self {
            winners: winners
                .split_ascii_whitespace()
                .map(|v| v.parse::<_>().unwrap())
                .collect::<Vec<_>>(),
            plays: plays
                .split_ascii_whitespace()
                .map(|v| v.parse::<_>().unwrap())
                .collect::<Vec<_>>(),
        }
    }
}
