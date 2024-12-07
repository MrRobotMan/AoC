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
                    if calibration(*validate, values, false) {
                        *validate
                    } else {
                        0
                    }
                })
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> String {
        output(
            self.equations
                .iter()
                .map(|(validate, values)| {
                    if calibration(*validate, values, true) {
                        *validate
                    } else {
                        0
                    }
                })
                .sum::<i64>(),
        )
    }
}

fn calibration(check: i64, values: &[i64], concat: bool) -> bool {
    match values.len() {
        2 => {
            values.iter().sum::<i64>() == check
                || values.iter().product::<i64>() == check
                || (concat && values[0] * 10_i64.pow(values[1].ilog10() + 1) + values[1] == check)
        }
        _ => {
            calibration(
                check,
                &[vec![values[0] + values[1]], values[2..].to_vec()].concat(),
                concat,
            ) || calibration(
                check,
                &[vec![values[0] * values[1]], values[2..].to_vec()].concat(),
                concat,
            ) || (concat
                && calibration(
                    check,
                    &[
                        vec![values[0] * 10_i64.pow(values[1].ilog10() + 1) + values[1]],
                        values[2..].to_vec(),
                    ]
                    .concat(),
                    concat,
                ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let expected = true;
        let actual = calibration(9, &[4, 5], false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_2() {
        let expected = true;
        let actual = calibration(25, &[4, 5, 5], false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_3() {
        let expected = true;
        let actual = calibration(292, &[11, 6, 16, 20], false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_4() {
        assert!(!calibration(156, &[15, 6], false));
        assert!(calibration(156, &[15, 6], true));
    }

    #[test]
    fn test_5() {
        assert!(calibration(7290, &[6, 8, 6, 15], true))
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

    #[test]
    fn test_example2() {
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
        let expected = "11387";
        let actual = day.part2();
        assert_eq!(expected, actual);
    }
}
