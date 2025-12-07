use std::collections::HashSet;

use puzlib::Vec2D;

fn main() {
    println!("---- 2025: 07 ----");
    let input = "aoc2025/inputs/day07.txt";
    println!("Parsing");
    let (grid, max_rows) = parse(input);
    println!("Part 1: {}", part1(&grid, max_rows));
    println!("Part 2: {}", part2(&grid));
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

fn part1(grid: &HashSet<(Vec2D<i64>, char)>, max_rows: usize) -> usize {
    let mut splits = 0;
    let mut beams = grid
        .iter()
        .filter_map(|(Vec2D(r, c), ch)| {
            if ch == &'S' {
                Some(Vec2D(*r, *c))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    'outer: loop {
        let mut next_beams = HashSet::new();
        for beam in beams {
            if beam.0 > max_rows as i64 {
                break 'outer;
            }
            let next_row = beam + Vec2D(1, 0);
            if grid.contains(&(next_row, '^')) {
                splits += 1;
                next_beams.insert(next_row + Vec2D(0, -1));
                next_beams.insert(next_row + Vec2D(0, 1));
            } else {
                next_beams.insert(next_row);
            };
        }
        beams = next_beams;
    }
    splits
}

fn part2(_model: &HashSet<(Vec2D<i64>, char)>) -> String {
    "Unsolved".into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 21;
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
        let actual = part1(&model.0, model.1);
        assert_eq!(expected, actual);
    }
}
