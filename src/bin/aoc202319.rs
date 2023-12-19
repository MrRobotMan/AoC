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
        // Parse the input
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

struct Rule {
    fields: char,
    comps: char,
    values: usize,
    workflow: Status,
}

enum Status {
    Accepted,
    Rejected,
    Workflow(String),
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn score(&self) -> usize {
        self.x + self.m + self.a + self.x
    }

    fn process_workflow(&self, workflow: &[Rule]) -> Status {
        todo!()
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
    fn test_parsing() {
    	let expected=;
    	let actual=;
    	assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
    	let expected=;
    	let actual=;
    	assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
    	let expected=;
    	let actual=;
    	assert_eq!(expected, actual);
    }
    
    
    
}
