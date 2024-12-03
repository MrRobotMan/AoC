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
        output(self.reports.iter().filter(|report| dampen(report)).count())
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

fn dampen(report: &[i64]) -> bool {
    if check_levels(report) {
        return true;
    }
    for index in 0..report.len() {
        let mut other = report.to_vec();
        other.remove(index);
        if check_levels(&other) {
            return true;
        }
    }
    false
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

    #[test]
    fn test_dampen() {
        let reports = [
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(4, reports.iter().filter(|r| dampen(r)).count());
    }
}
