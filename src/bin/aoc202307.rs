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
    hands: Vec<Hand>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn parse(&mut self) {
        for line in read_lines("inputs/2023/day07.txt") {
            self.hands.push(line.as_str().into())
        }
        for hand in &self.hands[..10] {
            println!("{hand:?}");
        }
        println!("{:?}", self.hands.last());
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: [u8; 5],
    bid: i32,
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
        Self {
            cards: cards.try_into().unwrap(),
            bid: bid.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parsing() {
        let expected = vec![
            Hand {
                cards: [3, 2, 10, 3, 13],
                bid: 765,
            },
            Hand {
                cards: [10, 5, 5, 11, 5],
                bid: 684,
            },
            Hand {
                cards: [13, 13, 6, 7, 7],
                bid: 28,
            },
            Hand {
                cards: [13, 10, 11, 11, 10],
                bid: 220,
            },
            Hand {
                cards: [12, 12, 12, 11, 14],
                bid: 483,
            },
        ];
        let actual = INPUT.lines().map(|l| l.into()).collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
}
