use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use puzlib::{Vec3D, read_number_lists};

fn main() {
    println!("---- 2025: 08 ----");
    let input = "aoc2025/inputs/day08.txt";
    println!("Parsing");
    let (heap, boxes) = parse(input);
    println!("Part 1: {}", part1(heap.clone(), &boxes, 1000));
    println!("Part 2: {}", part2(heap, &boxes));
}
type Points = BinaryHeap<Reverse<(MinNonNan, Vec3D<usize>, Vec3D<usize>)>>;

fn parse<S: AsRef<std::path::Path> + std::fmt::Display>(input: S) -> (Points, Vec<Vec3D<usize>>) {
    let mut heap = BinaryHeap::new();
    let mut boxes = vec![];
    let mut cur = 1;
    let points = read_number_lists::<S, usize>(input, ",")
        .iter()
        .map(|p| Vec3D(p[0], p[1], p[2]))
        .collect::<Vec<_>>();
    for point in &points[..points.len() - 1] {
        boxes.push(*point);
        for other in &points[cur..] {
            let p = point.map(|v| v as i32);
            let o = other.map(|v| v as i32);
            heap.push(Reverse((MinNonNan(p.distance_to(o)), *point, *other)))
        }
        cur += 1;
    }
    boxes.push(*points.last().unwrap());
    (heap, boxes)
}

fn part1(mut points: Points, boxes: &[Vec3D<usize>], items: usize) -> usize {
    let mut connected = boxes
        .iter()
        .map(|b| HashSet::from([*b]))
        .collect::<Vec<_>>();
    for _ in 0..items {
        let Reverse((_, a, b)) = points.pop().unwrap();
        let mut to_merge = HashSet::new();
        for (idx, circuit) in connected.iter().enumerate() {
            if circuit.contains(&a) {
                to_merge.insert(idx);
            }
            if circuit.contains(&b) {
                to_merge.insert(idx);
            }
        }
        if to_merge.is_empty() {
            connected.push(HashSet::from([a, b]));
        } else {
            let mut to_merge = to_merge.into_iter().collect::<Vec<_>>();
            to_merge.sort();
            for idx in to_merge.iter().skip(1).rev() {
                let add = connected[*idx].clone();
                connected.get_mut(to_merge[0]).unwrap().extend(add);
                connected.remove(*idx);
            }
        }
    }
    connected.sort_by_key(|b| std::cmp::Reverse(b.len()));
    connected.iter().take(3).fold(1, |acc, v| acc * v.len())
}

fn part2(mut points: Points, boxes: &[Vec3D<usize>]) -> usize {
    let mut connected = boxes
        .iter()
        .map(|b| HashSet::from([*b]))
        .collect::<Vec<_>>();
    let mut last_pair = (boxes[0], boxes[1]);
    while connected.len() > 1 {
        let Reverse((_, a, b)) = points.pop().unwrap();
        last_pair = (a, b);
        let mut to_merge = HashSet::new();
        for (idx, circuit) in connected.iter().enumerate() {
            if circuit.contains(&a) {
                to_merge.insert(idx);
            }
            if circuit.contains(&b) {
                to_merge.insert(idx);
            }
        }
        if to_merge.is_empty() {
            connected.push(HashSet::from([a, b]));
        } else {
            let mut to_merge = to_merge.into_iter().collect::<Vec<_>>();
            to_merge.sort();
            for idx in to_merge.iter().skip(1).rev() {
                let add = connected[*idx].clone();
                connected.get_mut(to_merge[0]).unwrap().extend(add);
                connected.remove(*idx);
            }
        }
    }
    last_pair.0.0 * last_pair.1.0
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
        let (heap, boxes) = parse("inputs/test.txt");
        let actual = heap.len();
        assert_eq!(190, actual);
        assert_eq!(40, part1(heap, &boxes, 10))
    }
}
