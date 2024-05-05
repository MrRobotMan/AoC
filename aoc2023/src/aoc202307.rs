use std::{cmp::Ordering, fmt::Display};

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub hands: Vec<Hand>,
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
        (2023, 7)
    }

    fn parse(&mut self) {
        for line in read_lines(&self.input) {
            self.hands.push(line.as_str().into())
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut hands = self.hands.clone();
        hands.sort();
        output(
            hands
                .iter()
                .enumerate()
                .map(|(idx, hand)| (idx + 1) as i32 * hand.bid)
                .sum::<i32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut hands = self.hands.iter().map(|h| h.jokers()).collect::<Vec<_>>();
        hands.sort();
        output(
            hands
                .iter()
                .enumerate()
                .map(|(idx, hand)| (idx + 1) as i32 * hand.bid)
                .sum::<i32>(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand {
    pub cards: [u8; 5],
    pub bid: i32,
    pub score: Score,
}

impl Hand {
    fn jokers(&self) -> Self {
        let mut cards = self.cards;
        for (i, c) in self.cards.iter().enumerate() {
            if c == &11 {
                cards[i] = 0;
            }
        }
        let score = Score::best(&cards);
        Self {
            cards,
            bid: self.bid,
            score,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.cards {
            write!(
                f,
                "{}",
                match c {
                    0 => 'J',
                    c if c < &10 => (*c + b'0') as char,
                    10 => 'T',
                    11 => 'J',
                    12 => 'Q',
                    13 => 'K',
                    14 => 'A',
                    _ => panic!(),
                }
            )?;
        }
        Ok(())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score as u8 == other.score as u8 {
            for pairs in self.cards.iter().zip(other.cards.iter()) {
                match pairs.0.cmp(pairs.1) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                    Ordering::Greater => return Ordering::Greater,
                }
            }
            Ordering::Equal
        } else {
            (self.score as u8).cmp(&(other.score as u8))
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| match c {
                c if c.is_ascii_digit() => c as u8 - b'0',
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                c => panic!("Unknown card {c}"),
            })
            .collect::<Vec<u8>>();
        let score = cards.as_slice().into();
        Self {
            cards: cards.try_into().unwrap(),
            bid: bid.parse().unwrap(),
            score,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Score {
    fn best(cards: &[u8]) -> Self {
        if !cards.contains(&0) {
            return cards.into();
        }
        let mut counts = [0; 15];
        for card in cards.iter() {
            counts[*card as usize] += 1;
        }
        let jokers = counts[0];
        counts[0] = 0;
        counts.sort();
        counts.reverse();
        counts[0] += jokers;
        match counts[..5] {
            [5, 0, 0, 0, 0] => Self::FiveOfAKind,
            [4, 1, 0, 0, 0] => Self::FourOfAKind,
            [3, 2, 0, 0, 0] => Self::FullHouse,
            [3, 1, 1, 0, 0] => Self::ThreeOfAKind,
            [2, 2, 1, 0, 0] => Self::TwoPair,
            [2, 1, 1, 1, 0] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!("Unknown Hand Type {:?}", &counts[..5]),
        }
    }
}

impl From<&[u8]> for Score {
    fn from(value: &[u8]) -> Self {
        let mut counts = [0; 15];
        for card in value.iter() {
            counts[*card as usize] += 1;
        }
        counts.sort();
        counts.reverse();
        match counts[..5] {
            [5, 0, 0, 0, 0] => Self::FiveOfAKind,
            [4, 1, 0, 0, 0] => Self::FourOfAKind,
            [3, 2, 0, 0, 0] => Self::FullHouse,
            [3, 1, 1, 0, 0] => Self::ThreeOfAKind,
            [2, 2, 1, 0, 0] => Self::TwoPair,
            [2, 1, 1, 1, 0] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!("Unknown Hand Type {:?}", &counts[..5]),
        }
    }
}
