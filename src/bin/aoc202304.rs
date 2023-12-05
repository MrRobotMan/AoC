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
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    winners: Vec<i64>,
    plays: Vec<i64>,
}

impl From<&String> for Card {
    fn from(value: &String) -> Self {
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
        (&value).into()
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
}
