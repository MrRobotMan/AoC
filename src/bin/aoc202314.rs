use std::collections::HashSet;

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day14.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    grid: Vec<Vec<char>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 14)
    }

    fn parse(&mut self) {
        self.grid = aoc::read_grid(&self.input);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = self.grid.clone();
        for row in 1..self.grid.len() {
            self.step_row(&mut grid, row, -1);
        }
        output(
            grid.iter()
                .rev()
                .enumerate()
                .map(|(scale, row)| (scale + 1) * row.iter().filter(|v| v == &&'O').count())
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grids = HashSet::new();
        let mut grid = self.grid.clone();
        let mut first_repeat_time = 0;
        let mut first_repeat = Vec::new();
        // Cycle until we find the first repeat.
        while grids.insert(grid.clone()) {
            self.step_cycle(&mut grid);
            first_repeat_time += 1;
            first_repeat = grid.clone()
        }
        // Go one more step so grids don't match
        self.step_cycle(&mut grid);
        let mut cycle_time = 1;

        // Find the next repeat to get cycle time.
        while grid != first_repeat {
            self.step_cycle(&mut grid);
            cycle_time += 1;
        }

        // Do math!
        let remaining_cycles = (1_000_000_000 - first_repeat_time) % cycle_time;

        // Do the last few cycles.
        for _ in 0..remaining_cycles {
            self.step_cycle(&mut grid);
        }

        output(
            grid.iter()
                .rev()
                .enumerate()
                .map(|(scale, row)| (scale + 1) * row.iter().filter(|v| v == &&'O').count())
                .sum::<usize>(),
        )
    }
}

impl AocDay {
    fn step_row(&self, grid: &mut [Vec<char>], row: usize, delta: i32) {
        let cur_row = grid[row].clone();
        for (col, rock) in cur_row.iter().enumerate() {
            if rock == &'O' {
                let mut cur = row;
                while grid[(cur as i32 + delta) as usize][col] == '.' {
                    cur = (cur as i32 + delta) as usize;
                    if cur == 0 || cur == self.grid.len() - 1 {
                        break;
                    }
                }
                if cur != row {
                    grid[cur][col] = 'O';
                    grid[row][col] = '.';
                }
            };
        }
    }

    fn step_col(&self, grid: &mut [Vec<char>], col: usize, delta: i32) {
        let cur_col = grid.iter().map(|r| r[col]).collect::<Vec<_>>();
        for (row, rock) in cur_col.iter().enumerate() {
            if rock == &'O' {
                let mut cur = col;
                while grid[row][(cur as i32 + delta) as usize] == '.' {
                    cur = (cur as i32 + delta) as usize;
                    if cur == 0 || cur == self.grid[0].len() - 1 {
                        break;
                    }
                }
                if cur != col {
                    grid[row][cur] = 'O';
                    grid[row][col] = '.';
                }
            };
        }
    }

    fn step_cycle(&self, grid: &mut [Vec<char>]) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        // Tilt North
        for row in 1..rows {
            self.step_row(grid, row, -1);
        }
        // Tilt West
        for col in 1..cols {
            self.step_col(grid, col, -1);
        }
        // Tilt South
        for row in 2..=rows {
            self.step_row(grid, rows - row, 1);
        }
        // Tilt East
        for col in 2..=cols {
            self.step_col(grid, cols - col, 1);
        }
    }
}

#[cfg(test)]
fn show_grid(grid: &[Vec<char>]) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        show_grid(&day.grid);
        let expected = 136;
        let actual = day.part1()[0].parse::<i32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 64;
        let actual = day.part2()[0].parse::<i32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
