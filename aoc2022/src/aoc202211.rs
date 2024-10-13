use aoc::runner::{output, Runner};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Default)]
pub struct AocDay {
    input: String,
    monkeys: Vec<Monkey>,
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
        (2022, 11)
    }

    fn parse(&mut self) {
        self.monkeys = aoc::read_string_records(&self.input)
            .iter()
            .map(|m| m.into())
            .collect();
    }

    fn part1(&mut self) -> String {
        output(self.simulate(20, false))
    }

    fn part2(&mut self) -> String {
        output(self.simulate(10_000, true))
    }
}

impl AocDay {
    fn simulate(&self, rounds: usize, mod_val: bool) -> i64 {
        let cnt = self.monkeys.len();
        let mut monkeys = self.monkeys.clone();
        if mod_val {
            let val = monkeys.iter().map(|m| m.test).product();
            for monkey in monkeys.iter_mut() {
                monkey.mod_val = val
            }
        }
        for _ in 0..rounds {
            for idx in 0..cnt {
                let items = monkeys[idx].throw(cnt);
                for (monk, item) in items.iter().enumerate() {
                    monkeys[monk].catch(item)
                }
            }
        }
        let mut scores = monkeys.iter().map(|m| m.items_thrown).collect::<Vec<i64>>();
        scores.sort_by(|m1, m2| m2.cmp(m1));
        scores[0] * scores[1]
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

impl<T: AsRef<str>> From<T> for Operation {
    fn from(value: T) -> Self {
        let (_, right) = value.as_ref().split_once('=').unwrap();
        let opes = right.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        match (opes[1], opes[2]) {
            ("+", "old") => Self::Multiply(2),
            ("*", "old") => Self::Square,
            ("+", v) => Self::Add(v.parse().unwrap()),
            ("*", v) => Self::Multiply(v.parse().unwrap()),
            _ => panic!("Parse error {}", value.as_ref()),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: i64,
    actions: (usize, usize),
    items_thrown: i64,
    mod_val: i64,
}

impl<T: AsRef<str>> From<T> for Monkey {
    fn from(value: T) -> Self {
        /* Example Statement
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        */
        let mut stmts = value.as_ref().split('\n');
        stmts.next(); // Throw away id line.
        let (_, items) = stmts.next().unwrap().split_once(':').unwrap();
        let items = items
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let (_, ope) = stmts.next().unwrap().split_once(':').unwrap();
        let operation = ope.into();
        let (_, test) = stmts.next().unwrap().split_once(':').unwrap();
        let test = get_number_at_end(test);
        let (_, if_true) = stmts.next().unwrap().split_once(':').unwrap();
        let (_, if_false) = stmts.next().unwrap().split_once(':').unwrap();
        let actions = (get_number_at_end(if_true), get_number_at_end(if_false));
        Self {
            items,
            operation,
            test,
            actions,
            items_thrown: 0,
            mod_val: 0,
        }
    }
}

impl Monkey {
    fn throw(&mut self, count: usize) -> Vec<Vec<i64>> {
        let mut monkeys = vec![Vec::new(); count];
        while let Some(item) = self.items.pop_front() {
            self.items_thrown += 1;
            let inspected = self.inspect(item);
            if inspected % self.test == 0 {
                monkeys[self.actions.0].push(inspected)
            } else {
                monkeys[self.actions.1].push(inspected)
            }
        }
        monkeys
    }

    fn inspect(&self, item: i64) -> i64 {
        let new = match self.operation {
            Operation::Add(v) => item + v,
            Operation::Multiply(v) => item * v,
            Operation::Square => item * item,
        };
        if self.mod_val == 0 {
            new / 3
        } else {
            new % self.mod_val
        }
    }

    fn catch(&mut self, items: &[i64]) {
        for item in items.iter() {
            self.items.push_back(*item);
        }
    }
}

fn get_number_at_end<T: FromStr>(string: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    string.split(' ').last().unwrap().parse::<T>().unwrap()
}
