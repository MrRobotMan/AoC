use aoc::{
    read_grid,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    grid: Grid,
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
        (2024, 4)
    }

    fn parse(&mut self) {
        self.grid = Grid::new(read_grid(&self.input));
    }

    fn part1(&mut self) -> String {
        output(self.grid.search())
    }

    fn part2(&mut self) -> String {
        output(self.grid.count_crosses())
    }
}

#[derive(Debug, Default)]
struct Grid {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
    diag_up_right: Vec<Vec<char>>,
    diag_down_right: Vec<Vec<char>>,
}

impl Grid {
    fn new(text: Vec<Vec<char>>) -> Self {
        let n_rows = text.len();
        let n_cols = text[0].len();
        let rows = text;
        let mut columns = vec![Vec::new(); n_cols];
        let mut diag_up_right = vec![Vec::new(); n_rows + n_cols - 1];
        let mut diag_down_right = vec![Vec::new(); n_rows + n_cols - 1];
        let mut highlighted = vec![Vec::new(); n_rows];

        for (row, line) in rows.iter().enumerate() {
            for (col, letter) in line.iter().enumerate() {
                columns[col].push(*letter);
                diag_up_right[row + col].push(*letter);
                diag_down_right[n_rows + col - row - 1].push(*letter);
                highlighted[row].push(letter.to_string());
            }
        }
        for row in &mut diag_up_right {
            row.reverse();
        }

        Self {
            rows,
            columns,
            diag_up_right,
            diag_down_right,
        }
    }

    fn search(&self) -> usize {
        let to_find = [vec!['X', 'M', 'A', 'S'], vec!['S', 'A', 'M', 'X']];
        let mut count = 0;
        for row in &self.rows {
            for window in row.as_slice().windows(4) {
                if to_find.contains(&window.to_vec()) {
                    count += 1;
                }
            }
        }
        for col in &self.columns {
            for window in col.as_slice().windows(4) {
                if to_find.contains(&window.to_vec()) {
                    count += 1;
                }
            }
        }
        for diag in &self.diag_up_right {
            for window in diag.as_slice().windows(4) {
                if to_find.contains(&window.to_vec()) {
                    count += 1;
                }
            }
        }
        for diag in &self.diag_down_right {
            for window in diag.as_slice().windows(4) {
                if to_find.contains(&window.to_vec()) {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_crosses(&self) -> usize {
        (1..self.rows.len() - 1)
            .map(|row| {
                (1..self.columns.len() - 1)
                    .map(|col| {
                        if self.rows[row][col] == 'A' && self.check_diags(row, col) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn check_diags(&self, row: usize, col: usize) -> bool {
        let negative_diag = (self.rows[row - 1][col - 1] == 'S'
            && self.rows[row + 1][col + 1] == 'M')
            || (self.rows[row - 1][col - 1] == 'M' && self.rows[row + 1][col + 1] == 'S');
        let positive_diag = (self.rows[row - 1][col + 1] == 'S'
            && self.rows[row + 1][col - 1] == 'M')
            || (self.rows[row - 1][col + 1] == 'M' && self.rows[row + 1][col - 1] == 'S');
        negative_diag && positive_diag
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = Grid::new(vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]);
        let expected = 18;
        let actual = grid.search();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let grid = Grid::new(vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]);
        let expected = 9;
        let actual = grid.count_crosses();
        assert_eq!(expected, actual);
    }
}
