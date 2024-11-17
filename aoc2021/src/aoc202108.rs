use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    display: Vec<Segment>,
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
        (2021, 8)
    }

    fn parse(&mut self) {
        self.display = read_lines(&self.input).iter().map(Segment::from).collect();
    }

    fn part1(&mut self) -> String {
        output(self.display.iter().map(|d| d.count_unique()).sum::<usize>())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

const PATTERNS: [&str; 10] = [
    "abcefg",  // 0
    "cf",      // 1
    "acdeg",   // 2
    "acdfg",   // 3
    "bcdf",    // 4
    "abdfg",   // 5
    "abdefg",  // 6
    "acf",     // 7
    "abcdefg", // 8
    "abcdfg",  // 9
];

#[derive(Debug, Default)]
struct Segment {
    signal_patterns: Vec<String>,
    target_value: Vec<String>,
}

impl Segment {
    fn count_unique(&self) -> usize {
        self.target_value
            .iter()
            .filter(|v| [2, 3, 4, 7].contains(&v.len()))
            .count()
    }
}

impl From<&String> for Segment {
    fn from(value: &String) -> Self {
        let (input, output) = value.split_once(" | ").unwrap();
        let signal_patterns = input.split_whitespace().map(String::from).collect();
        let target_value = output.split_whitespace().map(String::from).collect();
        Self {
            signal_patterns,
            target_value,
        }
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
