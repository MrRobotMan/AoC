use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    equations: Vec<(i64, Vec<i64>)>,
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
        (2024, 7)
    }

    fn parse(&mut self) {
        read_lines(&self.input).iter().for_each(|line| {
            let (result, values) = line.split_once(':').unwrap();
            self.equations.push((
                result.parse().unwrap(),
                values
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            ));
        });
    }

    fn part1(&mut self) -> String {
        output(
            self.equations
                .iter()
                .map(|(validate, values)| {
                    if calibration(values).iter().any(|v| v == validate) {
                        *validate
                    } else {
                        0
                    }
                })
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn calibration(values: &[i64]) -> Vec<i64> {
    match values.len() {
        2 => vec![values[0] + values[1], values[0] * values[1]],
        _ => {
            let sub_res = calibration(&values[..2]);
            let mut left = vec![sub_res[0]];
            left.extend(values[2..].iter());
            let mut right = vec![sub_res[1]];
            right.extend(values[2..].iter());
            [calibration(&left), calibration(&right)].concat()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let expected = vec![9, 20];
        let actual = calibration(&[4, 5]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_2() {
        let expected = vec![14, 45, 25, 100];
        let actual = calibration(&[4, 5, 5]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_3() {
        let expected = vec![53, 660, 292, 5440, 102, 1640, 1076, 21120];
        let actual = calibration(&[11, 6, 16, 20]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example() {
        let mut day = AocDay {
            equations: vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20]),
            ],
            ..Default::default()
        };
        let expected = "3749";
        let actual = day.part1();
        assert_eq!(expected, actual);
    }
}
