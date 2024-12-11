use std::{collections::HashMap, ops::Range};

use aoc::{
    read_line,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
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
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 9)
    }

    fn parse(&mut self) {
        let mut block_start = 0;
        for (idx, ch) in read_line(&self.input).iter().enumerate() {
            let block = (*ch as u8 - b'0') as usize;
            match idx % 2 {
                0 => {
                    self.files.insert(block_start..block_start + block, idx / 2);
                }
                1 => self.open_ranges.push(block_start..block_start + block),
                n => unreachable!("How did {idx}%2 equal {n}"),
            }
            block_start += block;
        }
    }

    fn part1(&mut self) -> String {
        let mut files = self.files.clone();
        fragment(&self.open_ranges, &mut files);
        output(checksum(&files))
    }

    fn part2(&mut self) -> String {
        let mut files = self.files.clone();
        defrag(self.open_ranges.clone(), &mut files);
        output(checksum(&files))
    }
}

fn checksum(files: &HashMap<Range<usize>, usize>) -> usize {
    files
        .iter()
        .map(|(r, f)| r.clone().map(|idx| idx * f).sum::<usize>())
        .sum()
}

fn fragment(open_ranges: &[Range<usize>], files: &mut HashMap<Range<usize>, usize>) {
    for range in open_ranges {
        let mut start = range.start;
        let end = range.end;
        while start != end {
            let last = files
                .keys()
                .max_by(|r1, r2| r1.end.cmp(&r2.end))
                .unwrap()
                .clone();
            if last.end <= start {
                return;
            }
            let file = files.remove(&last).unwrap();
            if last.len() <= end - start {
                files.insert(
                    Range {
                        start,
                        end: start + last.len(),
                    },
                    file,
                );
                start += last.len();
            } else {
                files.insert(start..end, file);
                files.insert(
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

fn defrag(mut open_ranges: Vec<Range<usize>>, files: &mut HashMap<Range<usize>, usize>) {
    let mut ranges = files.keys().cloned().collect::<Vec<_>>();
    ranges.sort_by(|l, r| r.end.cmp(&l.end));
    'placed: for range in ranges {
        for idx in 0..open_ranges.len() {
            let open = &mut open_ranges[idx];
            if open.start > range.start {
                continue 'placed;
            }
            if open.len() >= range.len() {
                let file = files.remove(&range).unwrap();
                files.insert(open.start..open.start + range.len(), file);
                if open.len() > range.len() {
                    open.start += range.len();
                } else {
                    open_ranges.remove(idx);
                }
                continue 'placed;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new("2333133121414131402");
        day.parse();
        let actual = day.part1();
        assert_eq!("1928", actual);
        let actual = day.part2();
        assert_eq!("2858", actual);
    }
}
