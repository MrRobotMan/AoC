use aoc::runner::{output, run_solution, Runner};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day15.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    beacons: HashMap<(i32, i32), i32>,
    target_row: i32,
    target_range: (i32, i32),
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 15)
    }

    fn parse(&mut self) {
        for line in aoc::read_lines(&self.input) {
            if let Some((sensor, beacon)) = line.split_once(':') {
                let sensor = extract_coords(sensor);
                let beacon = extract_coords(beacon);
                self.beacons.insert(sensor, get_distance(&sensor, &beacon));
            };
        }
        self.target_row = 2_000_000;
        self.target_range = (0, 4_000_000);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        let mut crossings = HashSet::new();
        for ((x, y), distance) in self.beacons.iter() {
            let delta = distance - get_distance(&(*x, *y), &(*x, self.target_row));
            if delta < 0 {
                continue;
            } else {
                crossings.insert((x - delta, x + delta));
            }
        }

        let mut crossings = crossings.iter().cloned().collect::<Vec<(i32, i32)>>();
        crossings.sort();
        let (mut start, mut end) = crossings[0];
        for (s, e) in crossings.iter() {
            if *s > end {
                res += end - start;
                start = *s;
            }
            end = end.max(*e);
        }
        res += end - start;
        output(res)
    }

    fn part2(&mut self) -> Vec<String> {
        let scale = 4_000_000;
        for row in self.target_range.0..self.target_range.1 {
            let mut crossings = HashSet::new();
            for ((x, y), distance) in self.beacons.iter() {
                let delta = distance - get_distance(&(*x, *y), &(*x, row));
                if delta < 0 {
                    continue;
                } else {
                    crossings.insert((x - delta, x + delta));
                }
            }

            let mut crossings = crossings.iter().cloned().collect::<Vec<(i32, i32)>>();
            crossings.sort();
            let mut end = crossings[0].0;
            for (s, e) in crossings.iter() {
                if s > &end {
                    return output(scale * (*s as i64 - 1) + row as i64);
                }
                end = end.max(*e);
            }
        }
        output(scale)
    }
}

fn get_distance(start: &(i32, i32), other: &(i32, i32)) -> i32 {
    (start.0 - other.0).abs() + (start.1 - other.1).abs()
}

fn extract_coords(s: &str) -> (i32, i32) {
    let re = Regex::new(r"(?:=)(-?\d+)").unwrap();
    let v = re
        .captures_iter(s)
        .filter_map(|capture| capture.get(1))
        .map(|ma| ma.as_str().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (v[0], v[1])
}
