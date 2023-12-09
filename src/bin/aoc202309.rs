use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    histories: Vec<History>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 9)
    }

    fn parse(&mut self) {
        self.histories = aoc::read_lines("inputs/2023/day09.txt")
            .iter()
            .map(|l| l.into())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut last = self.histories.clone();
        for hist in last.iter_mut() {
            hist.build_next();
        }
        output(
            last.iter()
                .fold(0, |acc, hist| acc + hist.values.last().unwrap()),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, Default, Clone)]
struct History {
    values: Vec<i64>,
}

impl History {
    fn build_next(&mut self) {
        let mut temp = self.values.clone();
        let mut last = Vec::new();
        while !zeroes(&temp) {
            match temp.last() {
                Some(v) => last.push(*v),
                None => break,
            }
            temp = temp
                .as_slice()
                .windows(2)
                .map(|vals| vals[1] - vals[0])
                .collect();
        }
        last.reverse();
        self.values
            .push(last.into_iter().reduce(|acc, v| acc + v).unwrap());
    }
}
fn zeroes(values: &[i64]) -> bool {
    values.iter().all(|v| v == &0)
}

impl From<&String> for History {
    fn from(value: &String) -> Self {
        let values = value
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [[i64; 6]; 3] = [
        [0, 3, 6, 9, 12, 15],
        [1, 3, 6, 10, 15, 21],
        [10, 13, 16, 21, 30, 45],
    ];

    #[test]
    fn test_part1() {
        let expected = 114;
        let mut day = AocDay {
            histories: INPUT
                .iter()
                .map(|l| History { values: l.to_vec() })
                .collect(),
        };
        let actual = day.part1()[0].parse().unwrap_or(0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_build_next() {
        let expected = vec![1, 3, 6, 10, 15, 21, 28];
        let mut history = History {
            values: vec![1, 3, 6, 10, 15, 21],
        };
        history.build_next();
        let actual = history.values;
        assert_eq!(expected, actual);
    }
}
