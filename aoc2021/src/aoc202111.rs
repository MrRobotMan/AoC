use std::collections::HashSet;

use aoc::{
    read_grid_numbers,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    octopodes: Vec<u8>,
    grid: [usize; 2],
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn step(&mut self) -> usize {
        self.bump();
        let val = self.flash();
        self.set_to_zero();
        val
    }

    fn bump(&mut self) {
        for energy in self.octopodes.iter_mut() {
            *energy += 1;
        }
    }

    fn flash(&mut self) -> usize {
        let mut flashes = 0;
        let mut to_bump = self
            .octopodes
            .iter()
            .enumerate()
            .filter_map(|(idx, energy)| if *energy == 10 { Some(idx) } else { None })
            .collect::<HashSet<_>>();
        while !to_bump.is_empty() {
            let mut new = HashSet::new();
            for octopus in to_bump {
                flashes += 1;
                for neighbor in self.neighbors(octopus) {
                    self.octopodes[neighbor] += 1;
                    if self.octopodes[neighbor] == 10 {
                        new.insert(neighbor);
                    }
                }
            }
            to_bump = new;
        }
        flashes
    }

    fn neighbors(&self, location: usize) -> Vec<usize> {
        let [rows, cols] = self.grid;
        let (r, c) = ((location / cols), (location % cols));
        (r.saturating_sub(1)..(r + 2).min(rows))
            .flat_map(|row| {
                (c.saturating_sub(1)..(c + 2).min(cols)).filter_map(move |col| {
                    if (r, c) != (row, col) {
                        Some(row * cols + col)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn set_to_zero(&mut self) {
        for energy in self.octopodes.iter_mut() {
            if *energy > 9 {
                *energy = 0;
            }
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2021, 11)
    }

    fn parse(&mut self) {
        // Parse the input
        let grid = read_grid_numbers(&self.input);
        self.grid = [grid.len(), grid[0].len()];
        self.octopodes = grid.into_iter().flatten().collect();
    }

    fn part1(&mut self) -> String {
        output((0..100).map(|_| self.step()).sum::<usize>())
    }

    fn part2(&mut self) -> String {
        let mut steps = 100;
        while self.step() != self.grid[0] * self.grid[1] {
            steps += 1;
        }
        output(steps + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_neighbors_full() {
        let day = AocDay {
            octopodes: vec![
                1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1,
            ],
            grid: [5, 5],
            ..Default::default()
        };
        let expected = vec![0, 1, 2, 5, 7, 10, 11, 12];
        let actual = day.neighbors(6);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_neighbors_corner() {
        let day = AocDay {
            octopodes: vec![
                1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1,
            ],
            grid: [5, 5],
            ..Default::default()
        };
        let expected = vec![18, 19, 23];
        let actual = day.neighbors(24);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            octopodes: vec![
                1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1,
            ],
            grid: [5, 5],
            ..Default::default()
        };
        let expected_step_1 = &vec![
            3, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3, 4, 5, 4, 3,
        ];
        let mut flashes = day.step();
        let actual = &day.octopodes;
        assert_eq!(expected_step_1, actual);
        assert_eq!(9, flashes);
        let expected_step_2 = &vec![
            4, 5, 6, 5, 4, 5, 1, 1, 1, 5, 6, 1, 1, 1, 6, 5, 1, 1, 1, 5, 4, 5, 6, 5, 4,
        ];
        flashes += day.step();
        let actual = &day.octopodes;
        assert_eq!(expected_step_2, actual);
        assert_eq!(9, flashes);
    }

    #[test]
    fn test_full_ex() {
        let mut day = AocDay::new(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );
        day.parse();
        let expected = 1656;
        let actual = (0..100).map(|_| day.step()).sum::<usize>();
        assert_eq!(expected, actual);
        let mut steps = 100;
        while day.step() != day.grid[0] * day.grid[1] {
            steps += 1;
        }
        assert_eq!(steps + 1, 195);
    }
}
