use std::collections::{HashMap, VecDeque};

use aoc::{
    read_grid,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    vents: HashMap<(usize, usize), u8>,
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
        (2021, 9)
    }

    fn parse(&mut self) {
        for (row, line) in read_grid(&self.input).into_iter().enumerate() {
            for (col, chr) in line.into_iter().enumerate() {
                self.vents.insert((row, col), chr as u8 - 48);
            }
        }
    }

    fn part1(&mut self) -> String {
        output(self.get_low_point_values().iter().sum::<usize>())
    }

    fn part2(&mut self) -> String {
        output(self.get_basin_sizes().iter().take(3).product::<usize>())
    }
}

impl AocDay {
    fn get_low_point_values(&self) -> Vec<usize> {
        self.get_low_points()
            .iter()
            .map(|p| 1 + self.vents[p] as usize)
            .collect()
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        self.vents
            .iter()
            .filter_map(|(p, v)| {
                if self.get_neighbors(p).iter().all(|p2| self.vents[p2] > *v) {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_basin_sizes(&self) -> Vec<usize> {
        let targets = self.get_low_points();
        let mut basins = targets
            .iter()
            .map(|p| (*p, *p))
            .collect::<HashMap<(usize, usize), (usize, usize)>>();
        for vent in self.vents.keys() {
            if self.vents[vent] == 9 {
                continue;
            }
            self.bfs(vent, &targets, &mut basins);
        }
        let mut res = HashMap::new();
        for basin in basins.values() {
            res.entry(basin).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut res = res.values().copied().collect::<Vec<_>>();
        res.sort();
        res.reverse();
        res
    }
    fn bfs(
        &self,
        start: &(usize, usize),
        targets: &[(usize, usize)],
        basins: &mut HashMap<(usize, usize), (usize, usize)>,
    ) {
        let mut path = HashMap::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_front(*start);
        while let Some(node) = to_visit.pop_front() {
            if let Some(target) = basins.get(&node) {
                basins.insert(*start, *target);
                return;
            }
            for next_move in self.get_neighbors(&node) {
                if self.vents.get(&next_move) == Some(&9) || path.contains_key(&next_move) {
                    continue;
                }
                if let Some(target) = targets.iter().find(|p| **p == next_move) {
                    basins.insert(*start, *target);
                    return;
                }
                to_visit.push_back(next_move);
                path.insert(next_move, node);
            }
        }
    }

    fn get_neighbors(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];
        if let Some(left) = point.1.checked_sub(1)
            && self.vents.contains_key(&(point.0, left)) {
                res.push((point.0, left));
            };
        if self.vents.contains_key(&(point.0, point.1 + 1)) {
            res.push((point.0, point.1 + 1));
        }
        if let Some(up) = point.0.checked_sub(1)
            && self.vents.contains_key(&(up, point.1)) {
                res.push((up, point.1));
            };
        if self.vents.contains_key(&(point.0 + 1, point.1)) {
            res.push((point.0 + 1, point.1));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(String::from(
            "2199943210\n3987894921\n9856789892\n8767896789\n9899965678",
        ));
        day.parse();
        assert_eq!(15, day.get_low_point_values().iter().sum::<usize>());
        assert_eq!(
            1134,
            day.get_basin_sizes().iter().take(3).product::<usize>()
        );
    }
}
