use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
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
        (2022, 8)
    }

    fn parse(&mut self) {
        let mut rows = Vec::new();
        let mut columns = Vec::new();
        for (r, row) in aoc::read_lines(&self.input).iter().enumerate() {
            rows.push(Vec::new());
            for (c, val) in row.chars().enumerate() {
                if r == 0 {
                    columns.push(Vec::new());
                }
                let v = val
                    .to_string()
                    .parse::<u8>()
                    .expect("Unexpected parsing error.");
                rows[r].push(v);
                columns[c].push(v);
            }
        }
        let row_count = rows.len();
        let col_count = columns.len();
        self.grid = Grid {
            rows,
            columns,
            row_count,
            col_count,
        };
    }

    fn part1(&mut self) -> String {
        output(self.grid.visible_trees())
    }

    fn part2(&mut self) -> String {
        output(self.grid.tree_score())
    }
}

#[derive(Debug, Default)]
struct Grid {
    rows: Vec<Vec<u8>>,
    columns: Vec<Vec<u8>>,
    row_count: usize,
    col_count: usize,
}

impl Grid {
    fn visible_trees(&self) -> usize {
        let mut visible = 2 * (self.row_count + (self.col_count - 2)); // outside perimeter
        for row in 1..(self.row_count - 1) {
            for col in 1..(self.col_count - 1) {
                if Grid::is_visible(&self.rows[row], col)
                    || Grid::is_visible(&self.columns[col], row)
                {
                    visible += 1;
                }
            }
        }
        visible
    }

    fn tree_score(&self) -> usize {
        let mut max = 0;
        for row in 1..(self.row_count - 1) {
            for col in 1..(self.col_count - 1) {
                let val = Grid::tree_count(&self.rows[row], &self.columns[col], (row, col));
                if val > max {
                    max = val;
                }
            }
        }
        max
    }

    fn is_visible(trees: &[u8], tree: usize) -> bool {
        let (head, tail) = trees.split_at(tree);
        let (subject, tail) = tail.split_at(1);
        let subject = &subject[0];
        let mut from_head = true;
        let mut from_tail = true;
        for t in head.iter() {
            if t >= subject {
                from_head = false;
            }
        }
        for t in tail.iter() {
            if t >= subject {
                from_tail = false;
            }
        }
        from_head || from_tail
    }

    fn tree_count(row: &[u8], col: &[u8], tree: (usize, usize)) -> usize {
        let (row_head, row_tail) = row.split_at(tree.1);
        let (col_head, col_tail) = col.split_at(tree.0);
        let (tree, row_tail) = row_tail.split_at(1);
        let (_, col_tail) = col_tail.split_at(1);
        let tree = tree[0];
        let mut score = [0, 0, 0, 0];
        for t in row_head.iter().rev() {
            score[0] += 1;
            if t >= &tree {
                break;
            }
        }
        for t in row_tail.iter() {
            score[1] += 1;
            if t >= &tree {
                break;
            }
        }
        for t in col_head.iter().rev() {
            score[2] += 1;
            if t >= &tree {
                break;
            }
        }
        for t in col_tail.iter() {
            score[3] += 1;
            if t >= &tree {
                break;
            }
        }
        score[0] * score[1] * score[2] * score[3]
    }
}
