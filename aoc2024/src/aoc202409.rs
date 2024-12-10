use std::{collections::HashMap, ops::Range};

use aoc::{
    read_line,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    memory: Vec<usize>,
    open_ranges: Vec<Range<usize>>,
    files: HashMap<Range<usize>, usize>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn fragment(&mut self) {
        for range in &self.open_ranges {
            let mut start = range.start;
            let end = range.end;
            while start != end {
                let last = self
                    .files
                    .keys()
                    .max_by(|r1, r2| r1.end.cmp(&r2.end))
                    .unwrap()
                    .clone();
                if last.end <= start {
                    return;
                }
                let file = self.files.remove(&last).unwrap();
                if last.len() <= end - start {
                    self.files.insert(
                        Range {
                            start,
                            end: start + last.len(),
                        },
                        file,
                    );
                    start += last.len();
                } else {
                    self.files.insert(start..end, file);
                    self.files.insert(
                        Range {
                            start: last.start,
                            end: last.end - (end - start),
                        },
                        file,
                    );
                    start += end - start;
                }
            }
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 9)
    }

    fn parse(&mut self) {
        self.memory = read_line(&self.input)
            .iter()
            .map(|c| (*c as u8 - b'0') as usize)
            .collect();
        let mut block_start = 0;
        for (idx, block) in self.memory.iter().enumerate() {
            match idx % 2 {
                0 => {
                    self.files
                        .insert(block_start..block_start + *block, idx / 2);
                }
                1 => self.open_ranges.push(block_start..block_start + *block),
                n => unreachable!("How did {idx}%2 equal {n}"),
            }
            block_start += *block;
        }
    }

    fn part1(&mut self) -> String {
        self.fragment();
        output(
            self.files
                .iter()
                .map(|(r, f)| r.clone().map(|idx| idx * f).sum::<usize>())
                .sum::<usize>(),
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
        let mut day = AocDay::new("2333133121414131402");
        day.parse();
        let expected = "1928";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
