fn main() {
    println!("---- 2025: 05 ----");
    let input = puzlib::read_lines("aoc2025/inputs/day05.txt");
    println!("Parsing");
    let (ranges, ids) = parse(input);
    println!("Part 1: {}", part1(&ranges, &ids));
    println!("Part 2: {}", part2(&ranges, &ids));
}

fn parse(input: Vec<String>) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = vec![];
    let mut ids = vec![];
    for row in input {
        match row.split_once('-') {
            Some((start, end)) => ranges.push((start.parse().unwrap(), end.parse().unwrap())),
            None => ids.push(row.parse().unwrap()),
        }
    }
    ids.sort();
    ranges.sort();
    (ranges, ids)
}

fn part1(ranges: &[(usize, usize)], ids: &[usize]) -> usize {
    ids.iter().filter(|id| in_range(ranges, **id)).count()
}

fn part2(_ranges: &[(usize, usize)], _ids: &[usize]) -> String {
    "Unsolved".into()
}

fn in_range(ranges: &[(usize, usize)], id: usize) -> bool {
    for range in ranges {
        if range.0 <= id && id <= range.1 {
            return true;
        }
    }
    false
}
