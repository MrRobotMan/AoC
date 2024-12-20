use std::collections::{HashMap, HashSet};

use aoc::{
    read_grid_to_map,
    runner::{output, Runner},
    Vec2D, CARDINALS,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    maze: HashSet<Vec2D<i64>>,
    start: Vec2D<i64>,
    end: Vec2D<i64>,
    path: Vec<Vec2D<i64>>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn step(
        &self,
        cur: Vec2D<i64>,
        dir: Vec2D<i64>,
        visited: &HashMap<Vec2D<i64>, usize>,
    ) -> (Vec2D<i64>, Vec2D<i64>) {
        let mut next = cur + dir;
        if self.maze.contains(&next) {
            for new_dir in CARDINALS {
                next = cur + new_dir;
                if !visited.contains_key(&next) && !self.maze.contains(&next) {
                    return (next, new_dir);
                }
            }
        }
        (next, dir)
    }

    fn find_2ps_shortcut(
        &self,
        cur: Vec2D<i64>,
        visited: &HashMap<Vec2D<i64>, usize>,
    ) -> Option<Vec<usize>> {
        let mut shortcuts = vec![];
        for dir in CARDINALS {
            if self.maze.contains(&(cur + dir)) && visited.contains_key(&(cur + dir + dir)) {
                shortcuts.push(visited[&cur] - visited[&(cur + dir + dir)] - 2);
            }
        }
        match shortcuts.is_empty() {
            true => None,
            false => Some(shortcuts),
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 20)
    }

    fn parse(&mut self) {
        let points = read_grid_to_map(&self.input);
        for (loc, value) in points {
            match value {
                '#' => {
                    self.maze.insert(loc.into());
                }
                'S' => self.start = loc.into(),
                'E' => self.end = loc.into(),
                _ => (),
            }
        }
    }

    fn part1(&mut self) -> String {
        let mut visited = HashMap::new();
        let mut step = 0;
        let mut cur = self.start;
        let mut dir = CARDINALS[0];
        let mut shortcuts = HashMap::new();
        visited.insert(self.start, 0);
        while cur != self.end {
            (cur, dir) = self.step(cur, dir, &visited);
            step += 1;
            visited.insert(cur, step);
            if let Some(cheats) = self.find_2ps_shortcut(cur, &visited) {
                for cheat in cheats {
                    shortcuts.entry(cheat).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
        let mut path = visited.iter().collect::<Vec<_>>();
        path.sort_by_key(|v| v.1);
        self.path = path.iter().map(|(p, _)| **p).collect();
        output(
            shortcuts
                .iter()
                .filter_map(|(steps_saved, count)| {
                    if *steps_saved >= 100 {
                        Some(count)
                    } else {
                        None
                    }
                })
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        let mut cheats = 0;
        for start_pos in 0..self.path.len() {
            for end_pos in start_pos..self.path.len() {
                let start = self.path[start_pos];
                let end = self.path[end_pos];
                let cheat_dist = start.manhatten(&end);
                let path_dist = end_pos.saturating_sub(start_pos) as i64;
                let saved = path_dist - cheat_dist;
                if cheat_dist <= 20 && path_dist > cheat_dist && saved >= 100 {
                    cheats += 1;
                }
            }
        }
        output(cheats)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
        );
        day.parse();
        let expected = "0";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
