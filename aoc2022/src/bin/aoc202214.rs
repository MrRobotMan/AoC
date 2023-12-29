use aoc::runner::{output, run_solution, Runner};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day14.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    cave: Cave,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 14)
    }

    fn parse(&mut self) {
        let mut min_col = usize::MAX;
        let mut max_col = 0;
        let mut max_row = 0;
        let mut rocks = Vec::new();
        for path in aoc::read_lines(&self.input) {
            let mut v = Vec::new();
            for turn in path.split(" -> ") {
                if let Some((col, row)) = turn.split_once(',') {
                    let col = col.parse().unwrap();
                    let row = row.parse().unwrap();
                    max_col = max_col.max(col);
                    min_col = min_col.min(col);
                    max_row = max_row.max(row);
                    v.push((col, row))
                }
            }
            rocks.push(v);
        }
        self.cave = Cave::new(&rocks, (min_col, max_col), max_row, 500);
    }

    fn part1(&mut self) -> Vec<String> {
        self.cave.drop_sand();
        output(self.cave.grains)
    }

    fn part2(&mut self) -> Vec<String> {
        self.cave.set_floor();
        self.cave.drop_sand();
        output(self.cave.grains)
    }
}

#[derive(Debug, Default)]
struct Cave {
    nodes: Vec<Vec<char>>,
    sand_col: usize,
    width: usize,
    height: usize,
    grains: i32,
}

impl Cave {
    fn new(
        rocks: &[Vec<(usize, usize)>],
        col_range: (usize, usize),
        height: usize,
        sand_col: usize,
    ) -> Self {
        let col_offset = col_range.0 - 1;
        let sand_col = sand_col - col_offset;
        let width = col_range.1 - col_range.0 + 3;
        let mut nodes = vec![vec!['.'; width]; height + 1];
        for rock in rocks.iter() {
            for cur in 1..rock.len() {
                let prev = rock[cur - 1];
                let cur = rock[cur];
                if cur.0 as i32 - prev.0 as i32 != 0 {
                    // Moving horizontal
                    for col in ((cur.0).min(prev.0))..=((prev.0).max(cur.0)) {
                        nodes[cur.1][col - col_offset] = '#';
                    }
                } else {
                    // Moving vertical
                    for row in nodes
                        .iter_mut()
                        .take((prev.1).max(cur.1))
                        .skip((prev.1).min(cur.1))
                    {
                        row[cur.0 - col_offset] = '#';
                    }
                    // for row in ((cur.1).min(prev.1))..=((prev.1).max(cur.1)) {
                    //     nodes[row][cur.0 - col_offset] = '#';
                    // }
                }
            }
        }
        nodes[0][sand_col] = '+';

        Self {
            nodes,
            sand_col,
            width,
            height,
            grains: 0,
        }
    }

    fn drop_sand(&mut self) {
        let mut grain_location = (0, self.sand_col);
        while grain_location.0 < self.height
            && grain_location.1 > 0
            && grain_location.1 < self.width
        {
            grain_location = (0, self.sand_col);
            while grain_location.0 < self.height
                && grain_location.1 > 0
                && grain_location.1 < self.width
            {
                let row_below = (
                    self.nodes[grain_location.0 + 1][grain_location.1 - 1],
                    self.nodes[grain_location.0 + 1][grain_location.1],
                    self.nodes[grain_location.0 + 1][grain_location.1 + 1],
                );
                match row_below {
                    (_, '.', _) => grain_location.0 += 1, // Fall down
                    ('.', '#', _) | ('.', 'o', _) => {
                        // Fall down left
                        grain_location.1 -= 1;
                        grain_location.0 += 1
                    }
                    (_, '#', '.') | (_, 'o', '.') => {
                        // Fall down right
                        grain_location.1 += 1;
                        grain_location.0 += 1
                    }
                    _ => {
                        // Can't fall. Grain stays here.
                        self.nodes[grain_location.0][grain_location.1] = 'o';
                        self.grains += 1;
                        if grain_location.0 == 0 {
                            return;
                        }
                        break;
                    }
                }
            }
        }
    }

    fn set_floor(&mut self) {
        self.height += 2;
        let cur = self.width;
        let center = self.sand_col;
        self.width = 2 * (self.height + 1) + 1;
        self.sand_col = self.height + 1;
        let shift_right = self.height + 1 - center;
        let add_to_left = self.height + center + 2 - cur;
        for row in self.nodes.iter_mut() {
            let mut new = vec!['.'; shift_right];
            new.extend(row.clone());
            new.extend(vec!['.'; add_to_left]);
            *row = new;
        }
        self.nodes.push(vec!['.'; self.width]);
        self.nodes.push(vec!['#'; self.width]);
    }
}
