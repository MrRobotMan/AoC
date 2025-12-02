fn main() {
    println!("---- 2025: 02 ----");
    let input = puzlib::read_line_sep("aoc2025/inputs/day02.txt", ",");
    println!("Parsing");
    let model = parse(input);
    println!("Part 1: {}", part1(&model));
    println!("Part 2: {}", part2(&model));
}

fn parse(input: Vec<String>) -> Vec<(usize, usize)> {
    input
        .iter()
        .map(|pair| {
            let (left, right) = pair.split_once('-').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

fn part1(model: &[(usize, usize)]) -> usize {
    model
        .iter()
        .map(|(start, end)| count_invalid(*start, *end))
        .sum()
}

fn part2(_model: &[(usize, usize)]) -> String {
    "Unsolved".into()
}

fn count_invalid(start: usize, end: usize) -> usize {
    let mut invalid = 0;
    for n in start..=end {
        let s = format!("{n}");
        if s.len() % 2 == 1 {
            continue;
        }
        let mid = s.len() / 2;
        if s[..mid] == s[mid..] {
            invalid += n;
        }
    }
    invalid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 0;
        let actual = count_invalid(1698522, 1698528);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let expected = 1010;
        let actual = count_invalid(998, 1012);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example3() {
        let expected = 33;
        let actual = count_invalid(11, 22);
        assert_eq!(expected, actual);
    }
}
