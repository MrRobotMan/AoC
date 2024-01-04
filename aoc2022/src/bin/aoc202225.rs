use aoc::runner::{output, run_solution, Runner};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day25.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    burners: Vec<Snafu>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 25)
    }

    fn parse(&mut self) {
        self.burners = aoc::read_lines(&self.input)
            .iter()
            .map(|s| s.into())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug)]
struct Snafu {
    value: i64,
}

impl<T: AsRef<str>> From<T> for Snafu {
    fn from(value: T) -> Self {
        let value = value
            .as_ref()
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (pow, val)| {
                acc + (match val {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => unreachable!("Unknown val"),
                }) * 5_i64.pow(pow as u32)
            });
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let _total = 4890;
        let expected = "2=-10";
        let actual = &day.part1()[0];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 0;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
