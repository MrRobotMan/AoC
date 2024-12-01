use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
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
        // Parse the input
    }

    fn part1(&mut self) -> String {
        output("Unsolved")
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
        let expected = 0;
        let actual = 0;
        assert_eq!(expected, actual);
    }
}
        