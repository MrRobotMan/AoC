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
        .map(|(start, end)| count_invalid_exact(*start, *end))
        .sum()
}

fn part2(model: &[(usize, usize)]) -> usize {
    model
        .iter()
        .map(|(start, end)| count_invalid_all(*start, *end))
        .sum()
}

fn count_invalid_exact(start: usize, end: usize) -> usize {
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

fn count_invalid_all(start: usize, end: usize) -> usize {
    let mut invalid = 0;
    'outer: for n in start..=end {
        let s = format!("{n}");
        if s.len() < 2 {
            continue;
        }
        for sub in 1..=s.len() / 2 {
            if s == s[..sub].repeat(s.len() / sub) {
                invalid += n;
                continue 'outer;
            }
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
        let actual = count_invalid_exact(1698522, 1698528);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let expected = 1010;
        let actual = count_invalid_exact(998, 1012);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example3() {
        let expected = 33;
        let actual = count_invalid_exact(11, 22);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example4() {
        let expected = 1111111;
        let actual = count_invalid_all(1111111, 1111111);
        assert_eq!(expected, actual);
    }
}
