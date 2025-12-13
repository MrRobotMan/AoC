use std::collections::HashMap;

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
    find_paths(outputs, "you", "out", vec![], &mut HashMap::new())
}

fn part2(outputs: &HashMap<String, Vec<String>>) -> usize {
    find_paths(
        outputs,
        "svr",
        "out",
        vec![("dac".into(), false), ("fft".into(), false)],
        &mut HashMap::new(),
    )
}

type Cache<'a> = HashMap<(&'a str, Vec<(String, bool)>), usize>;

fn find_paths<'a>(
    outputs: &'a HashMap<String, Vec<String>>,
    start: &'a str,
    end: &'a str,
    required: Vec<(String, bool)>,
    cache: &mut Cache<'a>,
) -> usize {
    if let Some(result) = cache.get(&(start, required.clone())) {
        return *result;
    }
    if start == end {
        if required.iter().all(|v| v.1) {
            return 1;
        } else {
            return 0;
        };
    }
    let mut res = 0;
    for output in &outputs[start] {
        let mut required = required.clone();
        for (k, v) in required.iter_mut() {
            if k == output {
                *v = true;
            }
        }
        let val = find_paths(outputs, output, end, required.clone(), cache);
        cache.entry((output, required)).or_insert(val);
        res += val
    }
    res
}
