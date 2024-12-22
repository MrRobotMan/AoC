use aoc::{
    read_numbers,
    runner::{output, Runner},
};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    secrets: Vec<[i64; 2001]>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn generate_secrets(&mut self) {
        for idx in 0..2000 {
            for secret in self.secrets.iter_mut() {
                secret[idx + 1] = secret[idx].step();
            }
        }
    }

    fn get_best_change(&self) -> ([i64; 4], i64) {
        let mut groups = HashMap::new();
        for secret in &self.secrets {
            let mut found = HashSet::new();
            for group in secret.as_slice().windows(5) {
                let mut digits = [0; 4];
                let mut deltas = [0; 4];
                for (idx, pair) in group.windows(2).enumerate() {
                    digits[idx] = pair[1] % 10;
                    deltas[idx] = pair[1] % 10 - pair[0] % 10;
                }
                if found.insert(deltas) {
                    groups
                        .entry(deltas)
                        .and_modify(|v| *v += digits[3])
                        .or_insert(digits[3]);
                }
            }
        }
        let (k, v) = groups.iter().max_by_key(|(_, v)| **v).unwrap();
        (*k, *v)
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 22)
    }

    fn parse(&mut self) {
        self.secrets = read_numbers(&self.input)
            .iter()
            .map(|v| {
                let mut val = [0; 2001];
                val[0] = *v;
                val
            })
            .collect();
    }

    fn part1(&mut self) -> String {
        self.generate_secrets();
        output(self.secrets.iter().fold(0, |acc, v| acc + v[2000]))
    }

    fn part2(&mut self) -> String {
        output(self.get_best_change().1)
    }
}

trait Secret {
    fn mix(self, value: Self) -> Self;
    fn prune(self) -> Self;
    fn step(self) -> Self;
}

impl Secret for i64 {
    fn mix(self, value: Self) -> Self {
        self ^ value
    }

    fn prune(self) -> Self {
        self & ((1 << 24) - 1)
    }

    fn step(self) -> i64 {
        // mul 64 -> x = n<<6
        // mix -> n=x^n
        // prune -> n=n%16777216 (2^24) -> n & (1<<24)-1
        // div 32 -> >>5
        // mix
        // prune
        // mul 2048 -> <<11
        // mix
        // prune
        let mut secret = self;
        secret = (secret << 6).mix(secret).prune();
        secret = (secret >> 5).mix(secret).prune();
        (secret << 11).mix(secret).prune()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mix() {
        let expected = 37;
        let actual = 42.mix(15);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_prune() {
        let expected = 16113920;
        let actual = 100000000.prune();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example1() {
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut actual = vec![];
        let mut secret = 123;
        for _ in 0..10 {
            secret = secret.step();
            actual.push(secret);
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_sequence() {
        let expected = ([-2, 1, -1, 3], 23);
        let mut day = AocDay::default();
        for num in [1, 2, 3, 2024] {
            let mut v = [0; 2001];
            v[0] = num;
            day.secrets.push(v);
        }
        day.generate_secrets();
        let actual = day.get_best_change();
        assert_eq!(expected, actual);
    }
}
