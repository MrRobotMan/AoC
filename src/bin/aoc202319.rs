use std::collections::HashMap;

use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day19.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    parts: Vec<Part>,
    workflows: HashMap<String, Vec<Rule>>,
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

    fn part1(&mut self) -> Vec<String> {
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

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

fn parse_workflow(workflow: &str) -> (String, Vec<Rule>) {
    let (name, rules) = workflow.trim_end_matches('}').split_once('{').unwrap();
    (name.into(), rules.split(',').map(|s| s.into()).collect())
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Rule {
    field: char,
    comp: char,
    value: usize,
    workflow: Status,
    is_final: bool,
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
enum Status {
    Accepted,
    #[default]
    Rejected,
    Workflow(String),
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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
    fn score(&self) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse_workflow() {
        let expected = (
            "px".into(),
            vec![
                Rule {
                    field: 'a',
                    comp: '<',
                    value: 2006,
                    workflow: Status::Workflow("qkq".into()),
                    is_final: false,
                },
                Rule {
                    field: 'm',
                    comp: '>',
                    value: 2090,
                    workflow: Status::Accepted,
                    is_final: false,
                },
                Rule {
                    workflow: Status::Workflow("rfg".into()),
                    is_final: true,
                    ..Default::default()
                },
            ],
        );
        let actual = parse_workflow(INPUT.lines().next().unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_part() {
        let expected = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };
        let actual = "{x=787,m=2655,a=1222,s=2876}".into();
        assert_eq!(expected, actual);
        assert_eq!(7540, actual.score());
    }

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 19114;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let actual = 0;
        assert_eq!(expected, actual);
    }
}
