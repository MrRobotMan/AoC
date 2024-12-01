use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    left: Vec<i64>,
    right: Vec<i64>,
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
        for line in read_lines(&self.input) {
            let pair = line
                .split_whitespace()
                .map(|l| l.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            self.left.push(pair[0]);
            self.right.push(pair[1]);
        }
        self.left.sort();
        self.right.sort();
    }

    fn part1(&mut self) -> String {
        output(
            self.left
                .iter()
                .zip(self.right.iter())
                .map(|(l, r)| l.abs_diff(*r))
                .sum::<u64>(),
        )
    }

    fn part2(&mut self) -> String {
        output(
            self.left
                .iter()
                .map(|v| self.right.iter().filter(|p| *p == v).count() as i64 * v)
                .sum::<i64>(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        left.sort();
        right.sort();
        let mut day = AocDay {
            left,
            right,
            ..Default::default()
        };
        let expected = "11";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        left.sort();
        right.sort();
        let mut day = AocDay {
            left,
            right,
            ..Default::default()
        };
        let expected = "31";
        let actual = day.part2();
        assert_eq!(expected, actual);
    }
}
