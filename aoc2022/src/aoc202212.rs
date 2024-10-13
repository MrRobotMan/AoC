use std::collections::{HashSet, VecDeque};

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
        (2022, 12)
    }

    fn parse(&mut self) {
        let lines = aoc::read_grid(&self.input);
        self.grid.height = lines.len();
        self.grid.width = lines[0].len();
        for (row, line) in lines.iter().enumerate() {
            let mut v = line.iter().map(|c| *c as u8).collect::<Vec<u8>>();
            if let Some(col) = line.iter().position(|p| *p == 'S') {
                self.grid.start = (row, col);
                v[col] = b'a';
            }
            if let Some(col) = line.iter().position(|p| *p == 'E') {
                self.grid.end = (row, col);
                v[col] = b'z';
            }
            self.grid.nodes.push(v);
        }
    }

    fn part1(&mut self) -> String {
        // -2 beacuse the start and end nodes aren't included in the steps taken.
        output(self.grid.breadth_first(self.grid.start).len() - 2)
    }

    fn part2(&mut self) -> String {
        let mut paths = Vec::new();
        for (r, row) in self.grid.nodes.iter().enumerate() {
            for (c, node) in row.iter().enumerate() {
                if node == &b'a' {
                    paths.push(self.grid.breadth_first((r, c)));
                }
            }
        }
        output(
            paths
                .iter()
                .filter_map(|v| if v.is_empty() { None } else { Some(v.len()) })
                .min()
                .unwrap()
                - 2, // Start and end points removed from path len
        )
    }
}

const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Debug, Default, Clone)]
struct Grid {
    nodes: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl Grid {
    fn get_adjacent(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let ipos = (pos.0 as i32, pos.1 as i32);
        let height = self.height as i32;
        let width = self.width as i32;
        DIR.iter()
            .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
            .filter(|p| p.0 >= 0 && p.0 < height && p.1 >= 0 && p.1 < width)
            .map(|p| (p.0 as usize, p.1 as usize))
            .collect()
    }

    fn breadth_first(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = vec![vec![None; self.width]; self.height];
        queue.push_front(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            let points = self.get_adjacent(&node);
            let valid = points
                .iter()
                .filter(|next| self.nodes[next.0][next.1] <= self.nodes[node.0][node.1] + 1)
                .copied()
                .collect::<Vec<(usize, usize)>>();
            for pt in valid.iter() {
                if *pt == self.end {
                    parent[pt.0][pt.1] = Some(node);
                    queue.clear();
                } else if !visited.contains(pt) {
                    parent[pt.0][pt.1] = Some(node);
                    queue.push_back(*pt);
                    visited.insert(*pt);
                }
            }
        }

        let mut path = Vec::new();
        let mut cur = parent[self.end.0][self.end.1];
        while let Some(p) = cur {
            path.push(p);
            cur = parent[p.0][p.1];
        }
        path
    }
}
