use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day12.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    records: Vec<Record>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 12)
    }

    fn parse(&mut self) {
        self.records = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.into())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Default, Debug, PartialEq)]
struct Record {
    springs: Vec<char>,
    counts: Vec<i32>,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (springs, counts) = value.split_once(' ').unwrap();
        Self {
            springs: springs.chars().collect(),
            counts: counts.split(',').map(|c| c.parse().unwrap()).collect(),
        }
    }
}

impl From<&String> for Record {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

    #[test]
    fn test_parse_line() {
        let expected = Record {
            springs: "#.#.###".chars().collect(),
            counts: vec![1, 1, 3],
        };
        let actual = INPUT.lines().next().unwrap().into();
        assert_eq!(expected, actual);
    }
}
