use std::str::FromStr;

use puzlib::Vec2D;

fn main() {
    println!("---- 2025: 12 ----");
    let input = "aoc2025/inputs/day12.txt";
    println!("Parsing");
    let (blocks, regions) = parse(input);
    println!("Part 1: {}", part1(&blocks, &regions));
    println!("Part 2: {}", part2(&blocks, &regions));
}

fn parse<S: AsRef<std::path::Path> + std::fmt::Display>(
    input: S,
) -> (Vec<Vec<Vec2D<usize>>>, Vec<Region>) {
    let lines = puzlib::read_lines(input);
    let mut blocks = vec![];
    let mut reg = 0;
    let mut row = 0;
    let mut index = 0;
    for (line, contents) in lines.iter().enumerate() {
        // Got to start of regions section.
        if contents.contains('x') {
            reg = line;
            break;
        }
        if contents.contains(':') {
            index = contents[..contents.len() - 1].parse().unwrap();
            blocks.push(vec![]);
            row = 0;
        } else {
            let cur = blocks.get_mut(index).unwrap();
            cur.extend(contents.chars().enumerate().filter_map(|(col, ch)| {
                if ch == '#' {
                    Some(Vec2D(row, col))
                } else {
                    None
                }
            }));
            row += 1;
        }
    }
    (
        blocks,
        lines[reg..]
            .iter()
            .map(|line| line.parse().unwrap())
            .collect(),
    )
}

fn part1(blocks: &[Vec<Vec2D<usize>>], regions: &[Region]) -> usize {
    // Assume no complex packing. If total area > sum block areas it's ok.
    regions
        .iter()
        .filter(|reg| {
            reg.area()
                > reg
                    .blocks
                    .iter()
                    .enumerate()
                    .map(|(idx, count)| block_area(&blocks[idx]) * count)
                    .sum()
        })
        .count()
}

fn part2(_blocks: &[Vec<Vec2D<usize>>], _regions: &[Region]) -> String {
    "Unsolved".into()
}

struct Region {
    dimensions: (usize, usize),
    blocks: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }
}

impl FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dimensions, indices) = s.split_once(": ").unwrap();
        let dimensions = dimensions
            .split_once('x')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        let blocks = indices
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Self { dimensions, blocks })
    }
}

fn block_area(block: &[Vec2D<usize>]) -> usize {
    block.len()
}
