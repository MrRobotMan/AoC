use std::collections::{HashMap, VecDeque};

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
    cards: Vec<Card>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 4)
    }

    fn parse(&mut self) {
        self.cards = read_lines("inputs/2023/day04.txt")
            .iter()
            .map(|c| c.into())
            .collect::<Vec<Card>>();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.cards.iter().map(Card::score).sum::<i64>())
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.total_cards())
    }
}

impl AocDay {
    fn total_cards(&self) -> usize {
        let mut remaining = VecDeque::from((0..self.cards.len()).collect::<Vec<_>>());
        let mut counter = HashMap::new();
        for c in 0..self.cards.len() {
            counter.insert(c, 1);
        }
        while let Some(card) = remaining.pop_front() {
            for idx in 1..=self.cards[card].matches() {
                *counter.entry(card + idx).or_default() += 1;
                remaining.push_back(card + idx);
            }
        }
        counter.values().sum()
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    winners: Vec<i64>,
    plays: Vec<i64>,
}

impl Card {
    fn score(&self) -> i64 {
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

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, tail) = value.split_once(": ").unwrap();
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
impl From<String> for Card {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
impl From<&String> for Card {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pasrser() {
        let expected = Card {
            winners: vec![41, 48, 83, 86, 17],
            plays: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let actual = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .to_string()
            .into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let cards = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .split('\n')
            .map(|l| l.into())
            .collect::<Vec<Card>>();
        let expected = 13;
        let actual = cards.iter().map(Card::score).sum::<i64>();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_part2() {
        let cards = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .split('\n')
            .map(|l| l.into())
            .collect::<Vec<Card>>();
        let day = AocDay { cards };
        let expected = 30;
        let actual = day.total_cards();
        assert_eq!(expected, actual);
    }
}
