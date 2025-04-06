use std::collections::HashSet;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    algorithm: Vec<u8>,
    image: Image,
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
        (2021, 20)
    }

    fn parse(&mut self) {
        let mut lines = read_lines(&self.input);
        self.algorithm = lines
            .remove(0)
            .chars()
            .map(|ch| match ch {
                '#' => 1,
                _ => 0,
            })
            .collect();
        self.image = lines.into();
    }

    fn part1(&mut self) -> String {
        let mut infinte = vec![0];
        if self.algorithm[0] == 1 {
            infinte.push(1)
        }
        let mut infinity = infinte.iter().cycle();
        output(
            self.image
                .enchance(&self.algorithm, *infinity.next().unwrap())
                .enchance(&self.algorithm, *infinity.next().unwrap())
                .image
                .len(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Image {
    image: HashSet<(i32, i32)>,
    bounds: [i32; 4], // min row, max row, min col, max col
}

impl Image {
    fn enchance(&self, algorithm: &[u8], infinity: usize) -> Self {
        let mut enchanced = Self {
            image: HashSet::new(),
            bounds: [
                self.bounds[0] - 1,
                self.bounds[1] + 1,
                self.bounds[2] - 1,
                self.bounds[3] + 1,
            ],
        };
        for row in enchanced.bounds[0]..=enchanced.bounds[1] {
            for col in enchanced.bounds[2]..=enchanced.bounds[3] {
                if algorithm[self.pixel_value((row, col), infinity)] == 1 {
                    enchanced.image.insert((row, col));
                }
            }
        }
        enchanced
    }

    fn pixel_value(&self, target: (i32, i32), infinity: usize) -> usize {
        let mut res = 0;
        for (shift, offset) in INDICIES.iter().enumerate() {
            let loc = (target.0 + offset.0, target.1 + offset.1);
            if self.image.contains(&loc) {
                res += 1 << shift;
            } else if !(self.bounds[0]..=self.bounds[1]).contains(&loc.0)
                || !(self.bounds[2]..=self.bounds[3]).contains(&loc.1)
            {
                res += infinity << shift;
            }
        }
        res
    }
}

impl From<Vec<String>> for Image {
    fn from(lines: Vec<String>) -> Self {
        let mut image = Self::default();
        let mut rows = 0;
        let mut cols = 0;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    image.image.insert((row as i32, col as i32));
                }
                cols = cols.max(col);
            }
            rows = rows.max(row);
        }
        image.bounds = [0, rows as i32, 0, cols as i32];
        image
    }
}

// Start at bottom right to top left so we can enumerate to the correct bit shift.
const INDICIES: [(i32, i32); 9] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, 0),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pixel_value() {
        let expected = 34;
        let image = Image {
            image: HashSet::from([
                (0, 0),
                (0, 3),
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 4),
                (3, 2),
                (4, 2),
                (4, 3),
                (4, 4),
            ]),
            bounds: [0, 4, 0, 4],
        };
        let actual = image.pixel_value((2, 2), 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_enchance() {
        let algorithm = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#".chars().map(|ch| if ch == '#' {1} else {0}).collect::<Vec<_>>();
        let image = Image {
            image: HashSet::from([
                (0, 0),
                (0, 3),
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 4),
                (3, 2),
                (4, 2),
                (4, 3),
                (4, 4),
            ]),
            bounds: [0, 4, 0, 4],
        };

        let mut infinte = vec![0];
        if algorithm[0] == 1 {
            infinte.push(1);
        }
        let mut infinity = infinte.iter().cycle();
        let expected = 35;
        let actual = image
            .enchance(&algorithm, *infinity.next().unwrap())
            .enchance(&algorithm, *infinity.next().unwrap())
            .image
            .len();
        assert_eq!(expected, actual);
    }
}
