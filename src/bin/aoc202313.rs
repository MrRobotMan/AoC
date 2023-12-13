use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day13.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    patterns: Vec<Pattern>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 13)
    }

    fn parse(&mut self) {
        self.patterns = aoc::lines(&self.input)
            .split("\n\n")
            .map(Pattern::new)
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, PartialEq)]
struct Pattern {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

impl Pattern {
    fn new(pattern: &str) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_parsing() {
        let expected = Pattern {
            rows: vec![
                vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['#', '.', '#', '.', '#', '#', '.', '#', '.'],
            ],
            cols: vec![
                vec!['#', '.', '#', '#', '.', '.', '#'],
                vec!['.', '.', '#', '#', '.', '.', '.'],
                vec!['#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.'],
                vec!['.', '#', '.', '.', '#', '.', '#'],
                vec!['.', '#', '.', '.', '#', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.'],
                vec!['#', '#', '.', '.', '#', '#', '#'],
                vec!['.', '.', '#', '#', '.', '.', '.'],
            ],
        };
        let actual = Pattern::new(INPUT.split("\n\n").next().unwrap());

        assert_eq!(expected.rows, actual.rows);
        assert_eq!(expected.cols, actual.cols);
    }
}
