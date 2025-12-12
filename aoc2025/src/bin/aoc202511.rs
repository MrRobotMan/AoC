use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("---- 2025: 11 ----");
    let input = "aoc2025/inputs/day11.txt";
    println!("Parsing");
    let outputs = parse(input);
    println!("Part 1: {}", part1(&outputs));
    println!("Part 2: {}", part2(&outputs));
}

fn parse<S: AsRef<std::path::Path> + std::fmt::Display>(input: S) -> HashMap<String, Vec<String>> {
    puzlib::read_lines(input)
        .into_iter()
        .map(|line| {
            let (device, connections) = line.split_once(": ").unwrap();
            (
                device.into(),
                connections.split_whitespace().map(str::to_string).collect(),
            )
        })
        .collect()
}

fn part1(outputs: &HashMap<String, Vec<String>>) -> usize {
    find_paths(outputs, "you", "out").len()
}

fn part2(outputs: &HashMap<String, Vec<String>>) -> usize {
    // let svr_to_dac = find_paths(outputs, "svr", "dac");
    // let svr_to_fft = find_paths(outputs, "svr", "fft");
    find_paths(outputs, "svr", "out");
    0
}

fn find_paths<'a>(
    outputs: &'a HashMap<String, Vec<String>>,
    start: &'a str,
    end: &'a str,
) -> HashSet<Vec<&'a str>> {
    let mut res = HashSet::new();
    let mut queue: VecDeque<(&str, HashSet<&str>)> = VecDeque::new();
    queue.push_front((start, HashSet::new()));
    while let Some((device, visited)) = queue.pop_front() {
        for connection in &outputs[device] {
            if connection == end {
                let mut found = visited.clone();
                found.insert(end);
                res.insert(found.iter().copied().collect());
            } else {
                let mut next = visited.clone();
                if next.insert(connection) {
                    queue.push_back((connection, next));
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 0;
        let actual = 0;
        assert_eq!(expected, actual);
    }
}
