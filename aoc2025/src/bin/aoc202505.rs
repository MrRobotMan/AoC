fn main() {
    println!("---- 2025: 05 ----");
    let input = puzlib::read_lines("aoc2025/inputs/day05.txt");
    println!("Parsing");
    let (ranges, ids) = parse(input);
    println!("Part 1: {}", part1(&ranges, &ids));
    println!("Part 2: {}", part2(&ranges));
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

fn part2(ranges: &[(usize, usize)]) -> usize {
    let ranges = merge_ranges(ranges);
    ranges
        .iter()
        .fold(0, |acc, (start, end)| acc + (end - start + 1))
}

fn in_range(ranges: &[(usize, usize)], id: usize) -> bool {
    for range in ranges {
        if range.0 <= id && id <= range.1 {
            return true;
        }
    }
    false
}

fn merge_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut merged = vec![];
    let (mut start, mut end) = ranges[0];
    for range in ranges.iter().skip(1) {
        if range.0 <= end {
            end = end.max(range.1);
        } else {
            merged.push((start, end));
            (start, end) = *range;
        }
    }
    merged.push((start, end));
    merged
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merged() {
        let expected = vec![(1, 6), (10, 12), (21, 40)];
        let actual = merge_ranges(&[
            (1, 3),
            (2, 5),
            (5, 6),
            (10, 12),
            (21, 30),
            (25, 40),
            (30, 32),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_merged2() {
        let expected = vec![(3, 5), (10, 14), (16, 20)];
        let actual = merge_ranges(&[(3, 5), (10, 14), (11, 13), (16, 20), (12, 18)]);
        assert_eq!(expected, actual);
    }
}
