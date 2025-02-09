use std::collections::{HashMap, HashSet};

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    target: ((i64, i64), (i64, i64)),
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2021, 17)
    }

    fn parse(&mut self) {
        fn ends(data: &str) -> (i64, i64) {
            let (start, end) = data.split_once("..").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        }
        let input = &read_lines(&self.input)[0];
        let mut parts = input.split(", y=");
        let (_, x) = parts.next().unwrap().split_once("=").unwrap();
        let y = parts.next().unwrap();
        self.target = (ends(x), ends(y));
    }

    fn part1(&mut self) -> String {
        let mut times: HashMap<i64, [HashSet<i64>; 2]> = HashMap::new();
        let y_max = (self.target.1 .0).abs();
        for vel in 0..=y_max {
            for time in times_in_target(vel, -1, self.target.1, vert_pos_at_time, |d, _| {
                d >= self.target.1 .0
            }) {
                let time = times.entry(time).or_default();
                time[1].insert(vel);
            }
        }
        let t_max = *times.keys().max().unwrap();
        for vel in min_vel(self.target.0 .0)..=self.target.0 .1 {
            for time in times_in_target(vel, -1, self.target.0, horiz_pos_at_time, |d, t| {
                d <= self.target.0 .1 && t <= t_max
            }) {
                let time = times.entry(time).or_default();
                time[0].insert(vel);
            }
        }
        let max_y = times
            .values()
            .filter_map(|v| {
                if v.iter().all(|x| !x.is_empty()) {
                    Some(v[1].clone())
                } else {
                    None
                }
            })
            .flatten()
            .max()
            .unwrap();
        output(triangluar(max_y))
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn times_in_target(
    vel: i64,
    accel: i64,
    target: (i64, i64),
    dist: impl Fn(i64, i64, i64) -> i64,
    valid: impl Fn(i64, i64) -> bool,
) -> Vec<i64> {
    let mut res = vec![];
    let mut time = 0;
    loop {
        let d = dist(time, vel, accel);
        if !valid(d, time) {
            break;
        }
        if (target.0..=target.1).contains(&d) {
            res.push(time);
        }
        time += 1;
    }
    res
}

fn vert_pos_at_time(time: i64, vel: i64, accel: i64) -> i64 {
    if time <= vel {
        horiz_pos_at_time(time, vel, accel)
    } else if time <= 2 * vel + 1 {
        // Ex v = 3, t4 = t3, t5 = t2, t6 = t1, t7 = t0
        // Delta       1         3        5        7
        // 2 v + 1 -t  3      7 - 5 = 2   7-6=1
        horiz_pos_at_time(2 * vel + 1 - time, vel, accel)
    } else {
        triangluar(vel) - triangluar(time - vel - 1)
    }
}

fn horiz_pos_at_time(time: i64, vel: i64, accel: i64) -> i64 {
    triangluar(vel) + accel.signum() * triangluar(vel + time * accel)
}

fn min_vel(dist: i64) -> i64 {
    // Will stop horizontal motion at the triangluar number for the distance.
    let mut v = 0;
    while triangluar(v) < dist {
        v += 1
    }
    v
}

fn triangluar(num: i64) -> i64 {
    if num < 0 {
        return 0;
    }
    num * (num + 1) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_min_x_vel() {
        let target = (20, 30);
        let expected = 6;
        let actual = min_vel(target.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_x_distance_travelled() {
        let expected = vec![15, 21, 21, 21, 21];
        let actual = [3, 6, 7, 8, 9]
            .iter()
            .map(|t| horiz_pos_at_time(*t, 6, -1))
            .collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_y_distance_travelled() {
        let expected = vec![3, 6, 6, 5, 0, -9];
        let actual = [1, 3, 4, 5, 7, 9]
            .iter()
            .map(|t| vert_pos_at_time(*t, 3, -1))
            .collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_y_distance_travelled2() {
        let expected = -10;
        let actual = vert_pos_at_time(20, 9, -1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example1() {
        let mut day = AocDay::new("target area: x=20..30, y=-10..-5");
        day.parse();
        let expected = 45;
        let actual = day.part1().parse().unwrap();
        assert_eq!(expected, actual);
    }
}
