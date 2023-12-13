use core::panic;
use std::collections::HashMap;

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
    history: HashMap<Record, usize>,
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
        let records = self
            .records
            .iter()
            .map(|r| r.options(&mut self.history))
            .sum::<usize>();
        output(records)
    }

    fn part2(&mut self) -> Vec<String> {
        let records = self
            .records
            .iter()
            .map(|r| r.factor().options(&mut self.history))
            .sum::<usize>();
        output(records)
    }
}

#[derive(Default, Debug, PartialEq, Clone, Eq, Hash)]
struct Record {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl Record {
    fn factor(&self) -> Self {
        let mut springs = self.springs.clone();
        let mut groups = self.groups.clone();
        for _ in 0..4 {
            springs.push('?');
            springs.extend_from_slice(&self.springs);
            groups.extend_from_slice(&self.groups);
        }
        Self { springs, groups }
    }
    fn options(&self, history: &mut HashMap<Record, usize>) -> usize {
        if let Some(res) = history.get(self) {
            return *res;
        }
        match (self.springs.is_empty(), self.groups.is_empty()) {
            (true, true) => return 1,
            (true, false) => return 0,
            (false, true) => {
                if self.springs.iter().any(|c| c == &'#') {
                    return 0;
                } else {
                    return 1;
                }
            }
            (false, false) => (),
        }
        let mut res = 0;
        match self.springs[0] {
            '.' => {
                let new = Record {
                    springs: self.springs[1..].to_vec(),
                    groups: self.groups.clone(),
                };
                res += new.options(history);
            }
            '#' => {
                let group = self.groups[0];
                if self.springs.len() < group {
                    if history.insert(self.clone(), res).is_some() {
                        panic!("Tried to re-insert key");
                    };
                    return 0;
                }
                if self.springs[..group].iter().any(|c| c == &'.') {
                    if history.insert(self.clone(), res).is_some() {
                        panic!("Tried to re-insert key");
                    };
                    return 0;
                }
                if matches!(self.springs.get(group), Some('#')) {
                    if history.insert(self.clone(), res).is_some() {
                        panic!("Tried to re-insert key");
                    };
                    return 0;
                }
                let mut springs = self.springs.clone();
                if let Some(s) = springs.get_mut(group) {
                    *s = '.';
                }
                let new = Record {
                    springs: springs[group..].to_vec(),
                    groups: self.groups[1..].to_vec(),
                };
                res += new.options(history);
            }
            '?' => {
                let mut springs = self.springs.clone();
                springs[0] = '.';
                let good = Record {
                    springs: springs.clone(),
                    groups: self.groups.clone(),
                };
                res += good.options(history);
                springs[0] = '#';
                let bad = Record {
                    springs,
                    groups: self.groups.clone(),
                };
                res += bad.options(history);
            }
            c => panic!("Unknown record {c}"),
        }
        if history.insert(self.clone(), res).is_some() {
            panic!("Tried to re-insert key");
        };

        res
    }
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (springs, counts) = value.split_once(' ').unwrap();
        Self {
            springs: springs.chars().collect(),
            groups: counts.split(',').map(|c| c.parse().unwrap()).collect(),
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

    static INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_parse_line() {
        let expected = Record {
            springs: "???.###".chars().collect(),
            groups: vec![1, 1, 3],
        };
        let actual = INPUT.lines().next().unwrap().into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_options() {
        let record: Record = INPUT.lines().last().unwrap().into();
        let expected = 10;
        let mut history = HashMap::new();
        let actual = record.options(&mut history);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 21;
        let actual = day.part1()[0].parse::<i32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 525152;
        let actual = day.part2()[0].parse::<i32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
