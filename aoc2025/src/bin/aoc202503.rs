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

fn part2(_model: &[Vec<u8>]) -> String {
    "Unsolved".into()
}

fn turn_on_two(bank: &[u8]) -> usize {
    let max_loc = find_location(&bank[..bank.len() - 1]);
    let next_loc = find_location(&bank[max_loc + 1..]);
    (bank[max_loc] * 10 + bank[max_loc + 1 + next_loc]) as usize
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
}
