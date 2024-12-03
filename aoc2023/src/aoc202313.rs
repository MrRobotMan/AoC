use std::{collections::HashSet, fmt::Display};

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub patterns: Vec<Pattern>,
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
        (2023, 13)
    }

    fn parse(&mut self) {
        self.patterns = aoc::contents(&self.input)
            .split("\n\n")
            .map(Pattern::new)
            .collect();
    }

    fn part1(&mut self) -> String {
        output(self.patterns.iter().map(Pattern::score).sum::<usize>())
    }

    fn part2(&mut self) -> String {
        output(
            self.patterns
                .iter()
                .map(Pattern::smudge_score)
                .sum::<usize>(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub rows: Vec<Vec<char>>,
    pub cols: Vec<Vec<char>>,
}

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        let mut rows = Vec::new();
        let mut cols = vec![Vec::new(); pattern.lines().next().unwrap().len()];
        for line in pattern.lines() {
            let mut row = Vec::new();
            for (col, chr) in line.chars().enumerate() {
                row.push(chr);
                cols[col].push(chr);
            }
            rows.push(row)
        }
        Self { rows, cols }
    }

    pub fn score(&self) -> usize {
        if let Some(s) = self.base_score(None) {
            s
        } else {
            panic!("No mirror line found for\n{self}");
        }
    }

    fn base_score(&self, base: Option<usize>) -> Option<usize> {
        if let Some(row) = self.mirror_idx(Orientation::Rows, base) {
            Some(row * 100)
        } else {
            self.mirror_idx(Orientation::Cols, base)
        }
    }

    fn smudge_score(&self) -> usize {
        let base_score = self.score();
        if cfg!(test) {
            println!("Score = {base_score}");
            println!("{}", self.smudgable().len());
        }
        for option in self.smudgable() {
            if let Some(score) = option.base_score(Some(base_score)) {
                if score != base_score {
                    return score;
                }
                if base_score >= 100 {
                    if let Some(score) = option.mirror_idx(Orientation::Cols, Some(base_score)) {
                        return score;
                    }
                }
            }
        }
        panic!("No new score for\n{self}");
    }

    fn smudgable(&self) -> Vec<Self> {
        let mut options = Vec::new();
        for (idx, row) in self.rows.iter().enumerate() {
            for (line, other) in self.rows.iter().enumerate() {
                if line == idx {
                    continue;
                }
                let indices = row
                    .iter()
                    .enumerate()
                    .zip(other.iter())
                    .filter_map(|((idx, a), b)| if a != b { Some(idx) } else { None })
                    .collect::<Vec<_>>();
                if indices.len() == 1 {
                    let col = indices[0];
                    let mut rows = self.rows.clone();
                    let mut cols = self.cols.clone();
                    rows[idx][col] = other[col];
                    cols[col][idx] = other[col];
                    options.push(Self { rows, cols });
                }
            }
        }
        for (idx, col) in self.cols.iter().enumerate() {
            for (line, other) in self.cols.iter().enumerate() {
                if line == idx {
                    continue;
                }
                let indices = col
                    .iter()
                    .enumerate()
                    .zip(other.iter())
                    .filter_map(|((idx, a), b)| if a != b { Some(idx) } else { None })
                    .collect::<Vec<_>>();
                if indices.len() == 1 {
                    let row = indices[0];
                    let mut rows = self.rows.clone();
                    let mut cols = self.cols.clone();
                    rows[row][idx] = other[row];
                    cols[idx][row] = other[row];
                    options.push(Self { rows, cols });
                }
            }
        }
        options
    }

    fn mirror_idx(&self, orientation: Orientation, base: Option<usize>) -> Option<usize> {
        let (arr, base) = match orientation {
            Orientation::Rows => (&self.rows, base.map(|base| base / 100)),
            Orientation::Cols => (&self.cols, base),
        };
        let mut candidates = Vec::new();
        for (idx, loc) in arr.iter().enumerate() {
            let mut options = HashSet::new();
            for (line, other) in arr.iter().enumerate() {
                if line == idx {
                    continue;
                }
                if loc == other {
                    options.insert((idx + line) / 2);
                }
            }
            candidates.push(options);
        }

        let opts = candidates
            .iter()
            .fold(HashSet::new(), |mut acc: HashSet<usize>, o| {
                acc.extend(o);
                acc
            });

        for o in opts {
            let range = (candidates.len() - o - 1).min(o + 1);
            let left = o - (range - 1);
            let right = o + range + 1;
            let outside = (&candidates[..left], &candidates[right..]);
            if !outside.0.is_empty() && !outside.1.is_empty() {
                // Ony one outie can have sets.
                continue;
            }
            if candidates[left..right].iter().all(|c| c.contains(&o)) {
                let score = Some(o + 1);
                if base == score {
                    continue;
                }
                return score;
            }
        }
        None
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = String::new();
        for row in &self.rows {
            for chr in row {
                lines.push(*chr);
            }
            lines.push('\n');
        }
        write!(f, "{}", lines.trim())
    }
}

#[derive(Debug)]
enum Orientation {
    Rows,
    Cols,
}
