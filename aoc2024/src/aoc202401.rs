use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    lists: Vec<Vec<i64>>,
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
        (2024, 1)
    }

    fn parse(&mut self) {
        let mut left = vec![];
        let mut right = vec![];
        for line in read_lines(&self.input) {
            let pair = line
                .split_whitespace()
                .map(|l| l.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            left.push(pair[0]);
            right.push(pair[1]);
        }
        self.lists = vec![left, right];
    }

    fn part1(&mut self) -> String {
        let mut left = self.lists[0].clone();
        let mut right = self.lists[1].clone();
        left.sort();
        right.sort();
        output(
            left.iter()
                .zip(right.iter())
                .map(|(l, r)| l.abs_diff(*r))
                .sum::<u64>(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let left = [3, 4, 2, 1, 3, 3];
        let right = [4, 3, 5, 3, 9, 3];
        let mut day = AocDay {
            lists: vec![left.to_vec(), right.to_vec()],
            ..Default::default()
        };
        let expected = "11";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
