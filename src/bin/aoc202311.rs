use std::collections::HashSet;

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day11.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    galaxies: HashSet<(i32, i32)>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 11)
    }

    fn parse(&mut self) {
        let lines = aoc::read_chars(&self.input);
        let mut galaxies = HashSet::new();
        let mut cols = vec![1; lines[0].len()];
        let mut rows = vec![1; lines.len()];
        for (row, line) in lines.into_iter().enumerate() {
            for (col, chr) in line.iter().enumerate() {
                if matches!(chr, '#') {
                    cols[col] = 0;
                    rows[row] = 0;
                    galaxies.insert((row, col));
                }
            }
        }
        self.galaxies = galaxies
            .into_iter()
            .map(|g| {
                (
                    g.0 as i32 + rows[0..g.0].iter().sum::<i32>(),
                    g.1 as i32 + cols[0..g.1].iter().sum::<i32>(),
                )
            })
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    #[test]
    fn test_parsing() {
        let expected = HashSet::from_iter([
            (0, 4),
            (1, 9),
            (2, 0),
            (5, 8),
            (6, 1),
            (7, 12),
            (10, 9),
            (11, 0),
            (11, 5),
        ]);
        let mut day = AocDay {
            input: INPUT.to_string(),
            ..Default::default()
        };

        day.parse();
        let actual = day.galaxies;
        assert_eq!(expected, actual);
    }
}
