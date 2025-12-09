use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use puzlib::{Vec3D, read_number_lists};

fn main() {
    println!("---- 2025: 08 ----");
    let input = "aoc2025/inputs/day08.txt";
    println!("Parsing");
    let points = parse(input);
    println!("Part 1: {}", part1(points.clone(), 1000));
    println!("Part 2: {}", part2(points));
}
type Points = BinaryHeap<Reverse<(MinNonNan, Vec3D<usize>, Vec3D<usize>)>>;

fn parse<S: AsRef<std::path::Path> + std::fmt::Display>(input: S) -> Points {
    let mut heap = BinaryHeap::new();
    let mut cur = 1;
    let points = read_number_lists::<S, usize>(input, ",")
        .iter()
        .map(|p| Vec3D(p[0], p[1], p[2]))
        .collect::<Vec<_>>();
    for point in &points[..points.len() - 1] {
        for other in &points[cur..] {
            let p = point.map(|v| v as i32);
            let o = other.map(|v| v as i32);
            heap.push(Reverse((MinNonNan(p.distance_to(o)), *point, *other)))
        }
        cur += 1;
    }
    heap
}

fn part1(mut points: Points, items: usize) -> usize {
    let mut connected: Vec<HashSet<Vec3D<usize>>> = vec![];
    'item: for _ in 0..items {
        let Reverse((_, a, b)) = points.pop().unwrap();
        for circuit in connected.iter_mut() {
            if circuit.contains(&a) {
                circuit.insert(b);
                continue 'item;
            }
            if circuit.contains(&b) {
                circuit.insert(a);
                continue 'item;
            }
        }
        connected.push(HashSet::from([a, b]));
    }

    loop {
        let mut merged = false;
        let mut next: Vec<HashSet<Vec3D<usize>>> = vec![];
        'outer: for set in connected {
            for other in next.iter_mut() {
                if other.intersection(&set).count() > 0 {
                    other.extend(set);
                    merged = true;
                    continue 'outer;
                }
            }
            next.push(set);
        }
        connected = next;
        if !merged {
            break;
        }
    }
    connected.sort_by_key(|b| std::cmp::Reverse(b.len()));
    connected.iter().take(3).fold(1, |acc, v| acc * v.len())
}

fn part2(_model: Points) -> String {
    "Unsolved".into()
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct MinNonNan(f64);

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for MinNonNan {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let heap = parse("inputs/test.txt");
        let actual = heap.len();
        assert_eq!(190, actual);
        assert_eq!(40, part1(heap, 10))
    }
}
