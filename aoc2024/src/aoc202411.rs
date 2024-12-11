use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    stones: Vec<u64>,
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
        (2024, 11)
    }

    fn parse(&mut self) {
        self.stones = read_lines(&self.input)[0]
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> String {
        let mut stones = self.stones.clone();
        for _ in 0..25 {
            stones = blink(&stones)
        }
        output(stones.len())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn blink(stones: &[u64]) -> Vec<u64> {
    let mut res = vec![];
    for stone in stones {
        res.extend(split(*stone))
    }
    res
}

fn split(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let len = stone.ilog10() + 1;
    if len % 2 == 0 {
        let split = 10_u64.pow(len / 2);
        vec![stone / split, stone % split]
    } else {
        vec![stone * 2024]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_zero() {
        let expected = vec![1];
        let actual = split(0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_split_odd() {
        let expected = vec![2021976];
        let actual = split(999);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_split_even() {
        let expected = vec![42, 96];
        let actual = split(4296);
        assert_eq!(expected, actual);
        let expected = vec![100, 0];
        let actual = split(100000);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example() {
        let expected = 55312;
        let mut stones = vec![125, 17];
        for _ in 0..25 {
            stones = blink(&stones);
        }
        let actual = stones.len();
        assert_eq!(expected, actual);
    }
}
