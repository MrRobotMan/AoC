use std::collections::{HashMap, HashSet};

use puzlib::Vec2D;

fn main() {
    println!("---- 2025: 07 ----");
    let input = "aoc2025/inputs/day07.txt";
    println!("Parsing");
    let (grid, max_rows) = parse(input);
    let (splits, ends) = find_splits(&grid, max_rows);
    println!("Part 1: {splits}");
    println!("Part 2: {}", ends.iter().sum::<usize>());
}

fn parse<S: AsRef<std::path::Path> + std::fmt::Display>(
    input: S,
) -> (HashSet<(Vec2D<i64>, char)>, usize) {
    let mut grid = HashSet::new();
    let mut rows = 0;
    for ((r, c), ch) in puzlib::read_grid_to_map(input) {
        rows = rows.max(r);
        if ch != '.' {
            grid.insert((Vec2D(r as i64, c as i64), ch));
        }
    }
    (grid, rows)
}

fn find_splits(grid: &HashSet<(Vec2D<i64>, char)>, max_rows: usize) -> (usize, Vec<usize>) {
    let mut splits = 0;
    let mut beams = grid
        .iter()
        .filter_map(|(Vec2D(r, c), ch)| {
            if ch == &'S' {
                Some((Vec2D(*r, *c), 1))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    'outer: loop {
        let mut next_beams = HashMap::new();
        for (beam, cnt) in &beams {
            if beam.0 > max_rows as i64 {
                break 'outer;
            }
            let next_row = *beam + Vec2D(1, 0);
            if grid.contains(&(next_row, '^')) {
                splits += 1;
                let n = next_beams.entry(next_row + Vec2D(0, -1)).or_insert(0);
                *n += cnt;
                let n = next_beams.entry(next_row + Vec2D(0, 1)).or_insert(0);
                *n += cnt;
            } else {
                let n = next_beams.entry(next_row).or_insert(0);
                *n += cnt;
            };
        }
        beams = next_beams;
    }
    (splits, beams.values().cloned().collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected1 = 21;
        let expected2 = 40;
        let model = parse(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        let (actual1, actual2) = find_splits(&model.0, model.1);
        let actual2 = actual2.iter().sum::<usize>();
        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
    }
}
