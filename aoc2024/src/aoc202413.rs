use aoc::{
    read_string_records,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    machines: Vec<Machine>,
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
        (2024, 13)
    }

    fn parse(&mut self) {
        for record in read_string_records(&self.input) {
            self.machines.push(record.into());
        }
        println!("{:?}\n{:?}", self.machines[0], self.machines.last());
    }

    fn part1(&mut self) -> String {
        output(
            self.machines
                .iter()
                .filter_map(|m| m.tokens_needed(true))
                .map(|(a, b)| 3 * a + b)
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> String {
        output(
            self.machines
                .iter()
                .filter_map(|m| m.tokens_needed(false))
                .map(|(a, b)| 3 * a + b)
                .sum::<i64>(),
        )
    }
}

#[derive(Debug, Default)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn tokens_needed(&self, limit: bool) -> Option<(i64, i64)> {
        // px = a*ax + b*bx
        // py = a*ay + b*by
        // a = (px - b*bx)/ax
        // a = (py - b*by)/ay
        // (px - b*bx)/ax = (py - b*by)/ay
        // ay*(px - b*bx) = ax*(py - b*by)
        // ay*px - b*ay*bx = ax*py - b*ax*by
        // b*(ax*by - ay*bx) = ax*py - ay*px
        // b = (ax*py - ay*px) / (ax*by-ay*bx)
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (mut px, mut py) = self.prize;
        if !limit {
            px += 10_000_000_000_000;
            py += 10_000_000_000_000;
        }
        let b = (ax * py - ay * px) / (ax * by - ay * bx);
        let a = (px - b * bx) / ax;
        if a * ax + b * bx == px && a * ay + b * by == py {
            if limit && a > 100 && b > 100 {
                return None;
            }
            Some((a, b))
        } else {
            None
        }
    }
}

impl From<String> for Machine {
    fn from(value: String) -> Self {
        let values = value
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                let x: i64 = match left.split_once('+') {
                    Some((_, n)) => n.parse().unwrap(),
                    None => left.split_once('=').unwrap().1.parse().unwrap(),
                };
                let y: i64 = match right.split_once('+') {
                    Some((_, n)) => n.parse().unwrap(),
                    None => right.split_once('=').unwrap().1.parse().unwrap(),
                };
                (x, y)
            })
            .collect::<Vec<_>>();
        Self {
            button_a: values[0],
            button_b: values[1],
            prize: values[2],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_good_machine() {
        let expected = Some((80, 40));
        let machine = Machine {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        };
        let actual = machine.tokens_needed(true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bad_machine() {
        let expected = None;
        let machine = Machine {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748, 12176),
        };
        let actual = machine.tokens_needed(true);
        assert_eq!(expected, actual);
    }
}
