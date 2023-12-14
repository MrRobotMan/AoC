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
        for row in 0..self.grid.len() {
            self.step(&mut grid, row);
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
        output("Unsolved")
    }
}

impl AocDay {
    fn step(&self, grid: &mut [Vec<char>], row: usize) {
        let cur_row = grid[row].clone();
        for (col, rock) in cur_row.iter().enumerate() {
            if rock == &'O' {
                let mut cur = row;
                while cur > 0 && grid[cur - 1][col] == '.' {
                    cur -= 1;
                }
                if cur != row {
                    grid[cur][col] = 'O';
                    grid[row][col] = '.';
                }
            };
        }
    }
}

#[cfg(test)]
fn show_grid(grid: &Vec<Vec<char>>) {
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
}
