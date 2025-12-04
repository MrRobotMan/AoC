fn main() {
    println!("---- 2025: 03 ----");
    let input = puzlib::read_lines("aoc2025/inputs/day03.txt");
    println!("Parsing");
    let banks = parse(input);
    println!("Part 1: {}", part1(&banks));
    println!("Part 2: {}", part2(&banks));
}

fn parse(input: Vec<String>) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect())
        .collect()
}

fn part1(model: &[Vec<u8>]) -> usize {
    model.iter().map(|b| turn_on_two(b)).sum()
}

fn part2(model: &[Vec<u8>]) -> usize {
    model.iter().map(|b| turn_on_twelve(b)).sum()
}

fn turn_on_two(bank: &[u8]) -> usize {
    let max_loc = find_location(&bank[..bank.len() - 1]);
    let next_loc = find_location(&bank[max_loc + 1..]);
    (bank[max_loc] * 10 + bank[max_loc + 1 + next_loc]) as usize
}

fn turn_on_twelve(bank: &[u8]) -> usize {
    let mut locs = [0; 12];
    let last = bank.len() - locs.len() + 1;
    locs[0] = find_location(&bank[..last]);
    for idx in 1..12 {
        let prev = locs[idx - 1];
        locs[idx] = find_location(&bank[prev + 1..last + idx]) + prev + 1;
    }
    locs.iter()
        .fold(0, |acc, val| acc * 10 + bank[*val] as usize)
}

fn find_location(bank: &[u8]) -> usize {
    for n in (1..10).rev() {
        if let Some(loc) = bank.iter().position(|i| *i == n) {
            return loc;
        }
    }
    panic!("No valid numbers")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 92;
        let actual = turn_on_two(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let expected = 888911112111;
        let actual = turn_on_twelve(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
        assert_eq!(expected, actual);
    }
}
