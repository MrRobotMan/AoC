use aoc::{
    read_numbers,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    secrets: Vec<usize>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn generate_secrets(&self, steps: usize) -> usize {
        let mut generated = self.secrets.clone();
        for _ in 0..steps {
            generated = generated.iter().map(step_secret).collect();
        }
        generated.iter().sum()
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 22)
    }

    fn parse(&mut self) {
        self.secrets = read_numbers(&self.input);
    }

    fn part1(&mut self) -> String {
        output(self.generate_secrets(2000))
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

trait Secret {
    fn mix(self, value: Self) -> Self;
    fn prune(self) -> Self;
}

impl Secret for usize {
    fn mix(self, value: Self) -> Self {
        self ^ value
    }

    fn prune(self) -> Self {
        self & ((1 << 24) - 1)
    }
}

fn step_secret(secret: &usize) -> usize {
    // mul 64 -> x = n<<6
    // mix -> n=x^n
    // prune -> n=n%16777216 (2^24) -> n & (1<<24)-1
    // div 32 -> >>5
    // mix
    // prune
    // mul 2048 -> <<11
    // mix
    // prune
    let mut secret = *secret;
    secret = (secret << 6).mix(secret).prune();
    secret = (secret >> 5).mix(secret).prune();
    (secret << 11).mix(secret).prune()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut actual = vec![];
        let mut secret = 123;
        for _ in 0..10 {
            secret = step_secret(&secret);
            actual.push(secret);
        }
        assert_eq!(expected, actual);
    }

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
}
