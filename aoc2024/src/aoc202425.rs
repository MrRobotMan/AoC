use aoc::{
    read_grid_records,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>,
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
        (2024, 25)
    }

    fn parse(&mut self) {
        let schematics = read_grid_records(&self.input);
        for schematic in schematics {
            let mut pins = [0; 5];
            let is_lock = schematic[0].iter().all(|ch| *ch == '#');
            for row in schematic[1..schematic.len() - 1].iter() {
                for (col, ch) in row.iter().enumerate() {
                    if *ch == '#' {
                        pins[col] += 1;
                    }
                }
            }
            if is_lock {
                self.locks.push(pins);
            } else {
                self.keys.push(pins);
            }
        }
    }

    fn part1(&mut self) -> String {
        let matching = self
            .locks
            .iter()
            .map(|lock| self.keys.iter().filter(|key| fits(key, lock)).count())
            .sum::<usize>();
        output(matching)
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn fits(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    key.iter().zip(lock.iter()).all(|(kp, lp)| kp + lp < 6)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            locks: vec![[0, 5, 3, 4, 3], [1, 2, 0, 5, 3]],
            keys: vec![[5, 0, 2, 1, 3], [4, 3, 4, 0, 2], [3, 0, 2, 0, 1]],
            ..Default::default()
        };
        let expected = "3";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
