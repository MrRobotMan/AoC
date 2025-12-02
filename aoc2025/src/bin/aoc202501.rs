use std::str::FromStr;

use puzlib::read_lines;

fn main() {
    println!("---- 2025: 01 ----");
    let input = read_lines("aoc2025/inputs/day01.txt");
    println!("Parsing");
    let model = parse(&input);
    println!("Part 1: {}", part1(&model));
    println!("Part 2: {}", part2(&model));
}

fn parse(input: &[String]) -> Vec<Direction> {
    input.iter().map(|l| l.parse().unwrap()).collect()
}

fn part1(model: &[Direction]) -> usize {
    let mut pos = 50;
    model
        .iter()
        .filter_map(|dir| {
            pos = rotate(pos, *dir);
            if pos == 0 { Some(()) } else { None }
        })
        .count()
}

fn part2(_model: &[Direction]) -> String {
    "Unsolved".into()
}

fn rotate(start: i32, dir: Direction) -> i32 {
    match dir {
        Direction::Right(n) => (start + n) % 100,
        Direction::Left(n) => (start - n).rem_euclid(100),
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Right(i32),
    Left(i32),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = &s[..1];
        let n = s[1..].parse().unwrap();
        match d {
            "R" => Ok(Self::Right(n)),
            "L" => Ok(Self::Left(n)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 19;
        let actual = rotate(11, Direction::Right(8));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let expected = 98;
        let actual = rotate(1, Direction::Left(3));
        assert_eq!(expected, actual);
    }
}
