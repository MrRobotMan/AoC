use aoc::{
    reader::read_string_records,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    draws: Vec<i64>,
    boards: Vec<Board>,
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
        (2021, 4)
    }

    fn parse(&mut self) {
        let mut iter = read_string_records(&self.input).into_iter();
        self.draws = iter
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        self.boards = iter.map(|rec| rec.into()).collect();
    }

    fn part1(&mut self) -> String {
        output(self.play())
    }

    fn part2(&mut self) -> String {
        for board in self.boards.iter_mut() {
            board.reset()
        }
        output(self.play_until_end())
    }
}

impl AocDay {
    fn play(&mut self) -> i64 {
        for draw in &self.draws.clone() {
            for board in self.boards.iter_mut() {
                if board.check_number(*draw) {
                    return board.score(*draw);
                }
            }
        }
        panic!("No winner found")
    }

    fn play_until_end(&mut self) -> i64 {
        let mut skip = vec![];
        let length = self.boards.len();
        for draw in &self.draws.clone() {
            for (idx, board) in self.boards.iter_mut().enumerate() {
                if !skip.contains(&idx) && board.check_number(*draw) {
                    skip.push(idx)
                }
                if skip.len() == length {
                    return board.score(*draw);
                }
            }
        }
        panic!("No winner found")
    }
}

#[derive(Debug, Default)]
struct Board {
    rows: Vec<Vec<i64>>,
    marked: Vec<Vec<bool>>,
}

impl Board {
    fn check_number(&mut self, draw: i64) -> bool {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for (col_idx, num) in row.iter().enumerate() {
                if *num == draw {
                    self.marked[row_idx][col_idx] = true;
                    return self.check_for_win();
                }
            }
        }
        false
    }

    fn check_for_win(&self) -> bool {
        for row in &self.marked {
            if row.iter().all(|&v| v) {
                return true;
            }
        }
        for col in 0..self.marked[0].len() {
            if self.marked.iter().map(|r| r[col]).all(|v| v) {
                return true;
            }
        }

        false
    }

    fn reset(&mut self) {
        for row in self.marked.iter_mut() {
            for val in row.iter_mut() {
                *val = false;
            }
        }
    }

    fn score(&self, draw: i64) -> i64 {
        draw * self
            .marked
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(col_idx, val)| {
                        if !val {
                            Some(self.rows[row_idx][col_idx])
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .sum::<i64>()
    }
}

impl From<String> for Board {
    fn from(value: String) -> Self {
        let rows: Vec<_> = value
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect();
        let marked = vec![vec![false; rows[0].len()]; rows.len()];
        Self { rows, marked }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            input: String::new(),
            draws: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                Board {
                    rows: vec![
                        vec![22, 13, 17, 11, 0],
                        vec![8, 2, 23, 4, 24],
                        vec![21, 9, 14, 16, 7],
                        vec![6, 10, 3, 18, 5],
                        vec![1, 12, 20, 15, 19],
                    ],
                    marked: vec![vec![false; 5]; 5],
                },
                Board {
                    rows: vec![
                        vec![3, 15, 0, 2, 22],
                        vec![9, 18, 13, 17, 5],
                        vec![19, 8, 7, 25, 23],
                        vec![20, 11, 10, 24, 4],
                        vec![14, 21, 16, 12, 6],
                    ],
                    marked: vec![vec![false; 5]; 5],
                },
                Board {
                    rows: vec![
                        vec![14, 21, 17, 24, 4],
                        vec![10, 16, 15, 9, 19],
                        vec![18, 8, 23, 26, 20],
                        vec![22, 11, 13, 6, 5],
                        vec![2, 0, 12, 3, 7],
                    ],
                    marked: vec![vec![false; 5]; 5],
                },
            ],
        };
        assert_eq!(4512, day.play());
        assert_eq!(1924, day.play_until_end());
    }
}
