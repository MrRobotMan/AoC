use std::collections::HashSet;

use puzlib::{Dir, Vec2D};

fn main() {
    println!("---- 2025: 04 ----");
    let input = puzlib::read_grid_to_map("aoc2025/inputs/day04.txt");
    println!("Parsing");
    let rolls = parse(input);
    println!("Part 1: {}", part1(&rolls));
    println!("Part 2: {}", part2(&rolls));
}

fn parse(input: Vec<((usize, usize), char)>) -> HashSet<Vec2D<usize>> {
    input
        .iter()
        .filter_map(|(loc, ch)| {
            if *ch == '@' {
                Some((*loc).into())
            } else {
                None
            }
        })
        .collect()
}

fn part1(rolls: &HashSet<Vec2D<usize>>) -> usize {
    rolls
        .iter()
        .filter(|roll| {
            Dir::<usize>::compass(roll)
                .iter()
                .filter_map(|n| {
                    if let Some(a) = n
                        && rolls.contains(a)
                    {
                        Some(1)
                    } else {
                        None
                    }
                })
                .count()
                < 4
        })
        .count()
}

fn part2(rolls: &HashSet<Vec2D<usize>>) -> usize {
    let mut removed = 0;
    let mut rolls = rolls.clone();
    loop {
        let to_remove = find_removable(&rolls);
        removed += to_remove.len();
        rolls.retain(|roll| !to_remove.contains(roll));
        if to_remove.is_empty() {
            break;
        }
    }
    removed
}

fn find_removable(rolls: &HashSet<Vec2D<usize>>) -> Vec<Vec2D<usize>> {
    rolls
        .iter()
        .filter(|roll| {
            Dir::<usize>::compass(roll)
                .iter()
                .filter_map(|n| {
                    if let Some(a) = n
                        && rolls.contains(a)
                    {
                        Some(1)
                    } else {
                        None
                    }
                })
                .count()
                < 4
        })
        .cloned()
        .collect()
}
