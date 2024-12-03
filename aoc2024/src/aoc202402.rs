use aoc::{
    read_number_lists,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    reports: Vec<Vec<i64>>,
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
        (2024, 2)
    }

    fn parse(&mut self) {
        self.reports = read_number_lists(&self.input, " ");
    }

    fn part1(&mut self) -> String {
        output(
            self.reports
                .iter()
                .filter(|report| check_levels(report))
                .count(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn check_levels(report: &[i64]) -> bool {
    let sign = (report[1] - report[0]).signum();
    for pair in report.windows(2) {
        let diff = pair[1] - pair[0];
        if diff == 0 || diff.abs() > 3 || diff.signum() != sign {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_passing() {
        assert!(check_levels(&[7, 6, 4, 2, 1]));
    }

    #[test]
    fn test_failing_too_large_step() {
        assert!(!check_levels(&[1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_failing_change_direction() {
        assert!(!check_levels(&[1, 3, 2, 4, 5]));
    }

    #[test]
    fn test_failing_no_change() {
        assert!(!check_levels(&[8, 6, 4, 4, 1]));
    }
}
