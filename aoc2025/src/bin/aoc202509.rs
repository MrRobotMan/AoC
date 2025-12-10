use std::collections::HashMap;

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

fn part1(tiles: &[Vec2D<i64>]) -> i64 {
    let pair = tiles
        .choose(2)
        .max_by_key(|pair| pair[0].manhattan(pair[1]))
        .unwrap();
    area(&pair)
}

fn part2(tiles: &[Vec2D<i64>]) -> i64 {
    let pair = get_pairs(tiles);
    area(&[pair.0, pair.1])
}

fn get_pairs(tiles: &[Vec2D<i64>]) -> (Vec2D<i64>, Vec2D<i64>) {
    let min_col = tiles.iter().min_by_key(|t| t.0).unwrap().0;
    let mut pairs = tiles
        .choose(2)
        .map(|p| (p[0], p[1], p[0].manhattan(p[1])))
        .collect::<Vec<_>>();
    pairs.sort_by(|l, r| r.2.cmp(&l.2));
    let perimeter = perimeter(tiles);
    for (a, b, _) in pairs.iter() {
        if is_green(a, b, &perimeter, min_col) {
            return (*a, *b);
        }
    }
    panic!("No all area found.");
}

fn perimeter(tiles: &[Vec2D<i64>]) -> Perimeter {
    let mut prev = tiles.last().unwrap();
    let mut res = HashMap::new();
    for pair in tiles.windows(2) {
        let cur = &pair[0];
        let next = &pair[1];
        if cur.1 == next.1 {
            for col in cur.0.min(next.0) + 1..cur.0.max(next.0) {
                res.insert(Vec2D(col, cur.1), Tile::Horiz);
            }
        } else {
            for row in cur.1.min(next.1) + 1..cur.1.max(next.1) {
                res.insert(Vec2D(cur.0, row), Tile::Vert);
            }
        }
        res.insert(*cur, Tile::new(prev, cur, next));
        prev = cur;
    }
    let cur = tiles.last().unwrap();
    let start = &tiles[0];
    res.insert(*cur, Tile::new(prev, cur, start));
    if cur.1 == start.1 {
        for col in cur.0.min(start.0) + 1..cur.0.max(start.0) {
            res.insert(Vec2D(col, cur.1), Tile::Horiz);
        }
    } else {
        for row in cur.1.min(start.1) + 1..cur.1.max(start.1) {
            res.insert(Vec2D(cur.0, row), Tile::Vert);
        }
    }
    res
}

type Perimeter = HashMap<Vec2D<i64>, Tile>;

fn is_green(t1: &Vec2D<i64>, t2: &Vec2D<i64>, perimeter: &Perimeter, min_col: i64) -> bool {
    let start_col = t1.0.min(t2.0);
    let end_col = t1.0.max(t2.0);
    let start_row = t1.1.min(t2.1);
    let end_row = t1.1.max(t2.1);

    if start_col != end_col {
        for col in start_col + 1..end_col {
            if let Some(Tile::Vert) = perimeter.get(&Vec2D(col, start_row)) {
                return false;
            };
            if let Some(Tile::Vert) = perimeter.get(&Vec2D(col, end_row)) {
                return false;
            }
        }
    }

    if start_row != end_row {
        for row in start_row + 1..end_row {
            if let Some(Tile::Horiz) = perimeter.get(&Vec2D(start_col, row)) {
                return false;
            }
            if let Some(Tile::Horiz) = perimeter.get(&Vec2D(end_col, row)) {
                return false;
            }
        }
    }

    if corner_is_out(Vec2D(start_col, start_row), perimeter, min_col) {
        return false;
    }
    if corner_is_out(Vec2D(end_col, start_row), perimeter, min_col) {
        return false;
    }
    if corner_is_out(Vec2D(start_col, end_row), perimeter, min_col) {
        return false;
    }
    if corner_is_out(Vec2D(end_col, start_row), perimeter, min_col) {
        return false;
    }
    true
}

fn corner_is_out(corner: Vec2D<i64>, perimeter: &Perimeter, min_col: i64) -> bool {
    let mut out = true;
    let mut cur = None;
    let Vec2D(end_col, row) = corner;
    for c in min_col..=end_col {
        let t = perimeter.get(&Vec2D(c, row));
        match (out, t) {
            (_, Some(Tile::Vert)) => out = !out,
            (true, Some(Tile::NorthEast)) | (true, Some(Tile::SouthEast)) => {
                out = false;
                cur = t;
            }
            (false, Some(Tile::NorthEast)) if cur == Some(&Tile::SouthEast) => out = true,
            (false, Some(Tile::NorthWest)) if cur == Some(&Tile::NorthEast) => out = true,

            (false, Some(Tile::NorthEast)) | (false, Some(Tile::SouthEast)) => out = true,
            _ => (),
        }
    }
    out
}

fn area(pair: &[Vec2D<i64>]) -> i64 {
    let a = pair[0];
    let b = pair[1];
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NorthEast, // └
    NorthWest, // ┘
    SouthEast, // ┌
    SouthWest, // ┐
    Horiz,
    Vert,
}

impl Tile {
    fn new(prev: &Vec2D<i64>, cur: &Vec2D<i64>, next: &Vec2D<i64>) -> Self {
        let Vec2D(from_col, from_row) = cur - prev;
        let Vec2D(to_col, to_row) = next - cur;
        match (
            from_col.signum(),
            from_row.signum(),
            to_col.signum(),
            to_row.signum(),
        ) {
            (1, 0, 0, 1) | (0, -1, -1, 0) => Self::SouthWest,
            (1, 0, 0, -1) | (0, 1, -1, 0) => Self::NorthWest,
            (-1, 0, 0, 1) | (0, -1, 1, 0) => Self::SouthEast,
            (-1, 0, 0, -1) | (0, 1, 1, 0) => Self::NorthEast,
            (1, 0, 1, 0) | (-1, 0, -1, 0) | (0, 1, 0, 1) | (0, -1, 0, -1) => {
                panic!("Straight through found")
            }
            _ => unreachable!(),
        }
    }
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

    #[test]
    fn test_is_green() {
        let tiles = parse(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        let perimeter = perimeter(&tiles);
        show(&perimeter);
        assert!(is_green(&Vec2D(9, 5), &Vec2D(2, 3), &perimeter, 0));
    }

    #[test]
    fn test_not_green() {
        let tiles = parse(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        let perimeter = perimeter(&tiles);
        assert!(!is_green(&Vec2D(11, 7), &Vec2D(2, 3), &perimeter, 0));
    }

    #[test]
    fn test_part_2() {
        let expected = (Vec2D(9, 5), Vec2D(2, 3));
        let tiles = parse(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        let perimeter = perimeter(&tiles);
        show(&perimeter);
        let actual = get_pairs(&tiles);
        assert_eq!(expected, actual);
    }

    fn show(perimeter: &Perimeter) {
        for row in 0..9 {
            for col in 0..14 {
                print!(
                    "{}",
                    match perimeter.get(&Vec2D(col, row)) {
                        Some(Tile::NorthEast) => '└',
                        Some(Tile::NorthWest) => '┘',
                        Some(Tile::SouthEast) => '┌',
                        Some(Tile::SouthWest) => '┐',
                        Some(Tile::Vert) => '|',
                        Some(Tile::Horiz) => '-',
                        None => '.',
                    }
                )
            }
            println!()
        }
    }
}
