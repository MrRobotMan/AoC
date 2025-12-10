use puzlib::{Combinations, Vec2D, read_number_lists};

fn main() {
    println!("---- 2025: 09 ----");
    let input = "aoc2025/inputs/day09.txt";
    println!("Parsing");
    let tiles = parse(input);
    println!("Part 1: {}", part1(&tiles));
    println!("Part 2: {}", part2(&tiles));
}

fn parse<S: AsRef<std::path::Path> + std::fmt::Display>(input: S) -> Vec<Vec2D<i64>> {
    read_number_lists(input, ",")
        .into_iter()
        .map(|pair| Vec2D(pair[0], pair[1]))
        .collect()
}

fn part1(model: &[Vec2D<i64>]) -> i64 {
    let pair = model
        .choose(2)
        .max_by_key(|pair| pair[0].manhattan(pair[1]))
        .unwrap();
    area(&pair)
}

fn part2(_model: &[Vec2D<i64>]) -> String {
    "Unsolved".into()
}

fn area(pair: &[Vec2D<i64>]) -> i64 {
    let a = pair[0];
    let b = pair[1];
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 24;
        let actual = area(&[Vec2D(2, 5), Vec2D(9, 7)]);
        assert_eq!(expected, actual);
    }
}
