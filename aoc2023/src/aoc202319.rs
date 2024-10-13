use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub parts: Vec<Part>,
    pub workflows: HashMap<String, Vec<Rule>>,
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
        (2023, 19)
    }

    fn parse(&mut self) {
        let lines = aoc::lines(&self.input);
        let mut lines = lines.lines();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (key, flows) = parse_workflow(line);
            self.workflows.insert(key, flows);
        }
        self.parts = lines.map(|p| p.into()).collect();
    }

    fn part1(&mut self) -> String {
        output(
            self.parts
                .iter()
                .filter_map(|p| {
                    if p.process_workflow("in", &self.workflows) == Status::Accepted {
                        Some(p.score())
                    } else {
                        None
                    }
                })
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        output(self.split_workflows())
    }
}

impl AocDay {
    fn split_workflows(&self) -> usize {
        let mut accepted = 0;
        let mut queue = VecDeque::new();
        queue.push_front((
            Status::Workflow("in".into()),
            [1..4001, 1..4001, 1..4001, 1..4001],
        ));
        while let Some((status, ranges)) = queue.pop_front() {
            match status {
                Status::Accepted => {
                    accepted += ranges.iter().fold(1, |acc, v| acc * v.clone().count())
                }
                Status::Rejected => (),
                Status::Workflow(s) => {
                    let res = self.step_through_workflow(&self.workflows[&s], ranges);
                    for found in res {
                        queue.push_back(found);
                    }
                }
            }
        }
        accepted
    }

    fn step_through_workflow(
        &self,
        rules: &[Rule],
        mut ranges: Ranges,
    ) -> Vec<(Status, [Range<usize>; 4])> {
        let mut collected = Vec::new();
        for rule in rules {
            let (good, bad) = rule.split_ranges(ranges.clone());
            collected.push(good);
            ranges = bad;
        }
        collected
    }
}

pub fn parse_workflow(workflow: &str) -> (String, Vec<Rule>) {
    let (name, rules) = workflow.trim_end_matches('}').split_once('{').unwrap();
    (name.into(), rules.split(',').map(|s| s.into()).collect())
}

type Ranges = [Range<usize>; 4];

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Rule {
    pub field: char,
    pub comp: char,
    pub value: usize,
    pub workflow: Status,
    pub is_final: bool,
}

impl Rule {
    pub fn split_ranges(&self, ranges: Ranges) -> ((Status, Ranges), Ranges) {
        if self.is_final {
            return ((self.workflow.clone(), ranges), Ranges::default());
        }
        let idx = match self.field {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            c => panic!("Unknown field {}", c),
        };
        match self.comp {
            '>' => {
                if ranges[idx].start > self.value {
                    // Everything is greater
                    ((self.workflow.clone(), ranges), Ranges::default())
                } else if ranges[idx].end - 1 <= self.value {
                    // Everything is lower
                    ((self.workflow.clone(), Ranges::default()), ranges)
                } else {
                    // Somewhere inbetween
                    let mut greater = ranges.clone();
                    greater[idx] = (self.value + 1)..ranges[idx].end;
                    let mut lower = ranges.clone();
                    lower[idx] = ranges[idx].start..(self.value + 1);
                    ((self.workflow.clone(), greater), lower)
                }
            }
            '<' => {
                if ranges[idx].start >= self.value {
                    // Everything is greater
                    ((self.workflow.clone(), Ranges::default()), ranges)
                } else if ranges[idx].end <= self.value {
                    // Everything is lower
                    ((self.workflow.clone(), ranges), Ranges::default())
                } else {
                    // Somewhere inbetween
                    let mut greater = ranges.clone();
                    greater[idx] = self.value..ranges[idx].end;
                    let mut lower = ranges.clone();
                    lower[idx] = ranges[idx].start..self.value;
                    ((self.workflow.clone(), lower), greater)
                }
            }
            o => panic!("Unknown operation {}", o),
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        match value.split_once(':') {
            Some((rule, workflow)) => {
                let workflow = match workflow {
                    "A" => Status::Accepted,
                    "R" => Status::Rejected,
                    s => Status::Workflow(s.into()),
                };
                let mut chars = rule.chars();
                let field = chars.next().unwrap();
                let comp = chars.next().unwrap();
                let value = rule[2..].parse().unwrap();
                Rule {
                    field,
                    comp,
                    value,
                    workflow,
                    is_final: false,
                }
            }
            None => Rule {
                workflow: match value {
                    "A" => Status::Accepted,
                    "R" => Status::Rejected,
                    s => Status::Workflow(s.into()),
                },
                is_final: true,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum Status {
    Accepted,
    #[default]
    Rejected,
    Workflow(String),
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut part = Part::default();
        for field in value[1..value.len() - 1].split(',') {
            match field.split_once('=').unwrap() {
                ("x", v) => part.x = v.parse().unwrap(),
                ("m", v) => part.m = v.parse().unwrap(),
                ("a", v) => part.a = v.parse().unwrap(),
                ("s", v) => part.s = v.parse().unwrap(),
                _ => panic!("Can't process {}", field),
            }
        }
        part
    }
}

impl Part {
    pub fn score(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn process_workflow(&self, start: &str, workflows: &HashMap<String, Vec<Rule>>) -> Status {
        let mut status = Status::Workflow(start.into());
        while let Status::Workflow(ref k) = status {
            let workflow = &workflows[k];
            for rule in workflow.iter() {
                if rule.is_final {
                    status = rule.workflow.clone();
                    break;
                }
                let val = match rule.field {
                    'x' => self.x,
                    'm' => self.m,
                    'a' => self.a,
                    's' => self.s,
                    c => panic!("Unknown field {}", c),
                };
                let check = match rule.comp {
                    '>' => val > rule.value,
                    '<' => val < rule.value,
                    o => panic!("Unknown operation {}", o),
                };
                if check {
                    status = rule.workflow.clone();
                    break;
                }
            }
        }
        status
    }
}
