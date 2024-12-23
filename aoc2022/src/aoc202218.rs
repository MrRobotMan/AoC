use aoc::{
    runner::{output, Runner},
    Vec3D,
};
use std::collections::{HashSet, VecDeque};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    pub(crate) cubes: HashSet<Vec3D<i64>>,
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
        (2022, 18)
    }

    fn parse(&mut self) {
        self.cubes = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.split(',').map(|p| p.parse().unwrap()).collect())
            .collect();
    }

    fn part1(&mut self) -> String {
        let total = self.cubes.iter().fold(0, |acc, cube| {
            acc + DIR
                .iter()
                .filter(|d| !self.cubes.contains(&(*cube + **d)))
                .count()
        });
        output(total)
    }

    fn part2(&mut self) -> String {
        let (mut min_x, mut min_y, mut min_z) = (i64::MAX, i64::MAX, i64::MAX);
        let (mut max_x, mut max_y, mut max_z) = (i64::MIN, i64::MIN, i64::MIN);
        for cube in &self.cubes {
            min_x = min_x.min(cube.0);
            max_x = max_x.max(cube.0);
            min_y = min_y.min(cube.1);
            max_y = max_y.max(cube.1);
            min_z = min_z.min(cube.2);
            max_z = max_z.max(cube.2);
        }

        // Set bounding box 1 cube away from furthest or else we miss the
        // faces at the edges of the bounding box.
        min_x -= 1;
        max_x += 1;
        min_y -= 1;
        max_y += 1;
        min_z -= 1;
        max_z += 1;

        let mut queue = VecDeque::from([Vec3D(min_x, min_y, min_z)]);
        let mut seen = HashSet::new();
        let mut count = 0;
        while let Some(cube) = queue.pop_front() {
            if !seen.insert(cube) {
                continue;
            }

            for &dir in &DIR {
                let loc = cube + dir;
                // Check for outside bounding box
                if loc.0 < min_x
                    || loc.0 > max_x
                    || loc.1 < min_y
                    || loc.1 > max_y
                    || loc.2 < min_z
                    || loc.2 > max_z
                {
                    continue;
                }
                if self.cubes.contains(&loc) {
                    count += 1;
                } else {
                    queue.push_back(loc);
                }
            }
        }
        output(count)
    }
}

const DIR: [Vec3D<i64>; 6] = [
    Vec3D(0, 0, 1),
    Vec3D(0, 0, -1),
    Vec3D(0, 1, 0),
    Vec3D(0, -1, 0),
    Vec3D(1, 0, 0),
    Vec3D(-1, 0, 0),
];
