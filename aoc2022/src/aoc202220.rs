use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    pub(crate) encrypted_coordinates: Vec<(usize, i64)>,
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
        (2022, 20)
    }

    fn parse(&mut self) {
        self.encrypted_coordinates = aoc::read_numbers(&self.input)
            .into_iter()
            .enumerate()
            .collect();
    }

    fn part1(&mut self) -> String {
        let mut nums = self.encrypted_coordinates.clone();
        let total = nums.len() as i64 - 1;
        for v in &self.encrypted_coordinates {
            if v.1 == 0 {
                continue;
            }
            let pos = nums.iter().position(|loc| loc == v).unwrap();
            nums.remove(pos);
            let new_pos = (pos as i64 + v.1).rem_euclid(total);
            if new_pos == 0 {
                nums.push(*v);
            } else {
                nums.insert(new_pos as usize, *v);
            };
        }

        let idx = nums.iter().position(|loc| loc.1 == 0).unwrap();

        output(
            nums[(1000 + idx) % nums.len()].1
                + nums[(2000 + idx) % nums.len()].1
                + nums[(3000 + idx) % nums.len()].1,
        )
    }

    fn part2(&mut self) -> String {
        let key = 811589153;
        let mut nums = self
            .encrypted_coordinates
            .iter()
            .map(|v| (v.0, key * v.1))
            .collect::<Vec<_>>();
        let total = nums.len() as i64 - 1;
        for _ in 0..10 {
            for idx in 0..self.encrypted_coordinates.len() {
                let pos = nums.iter().position(|loc| loc.0 == idx).unwrap();
                if nums[pos].1 == 0 {
                    continue;
                }
                let v = nums.remove(pos);
                let new_pos = (pos as i64 + v.1).rem_euclid(total);
                if new_pos == 0 {
                    nums.push(v);
                } else {
                    nums.insert(new_pos as usize, v);
                };
            }
        }
        let idx = nums.iter().position(|loc| loc.1 == 0).unwrap();

        output(
            nums[(1000 + idx) % nums.len()].1
                + nums[(2000 + idx) % nums.len()].1
                + nums[(3000 + idx) % nums.len()].1,
        )
    }
}
