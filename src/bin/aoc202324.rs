use aoc::{
    runner::{output, run_solution, Runner},
    Point3D,
};

use itertools::Itertools;

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day24.txt".into(),
        lower_limit: 200_000_000_000_000.,
        upper_limit: 400_000_000_000_000.,
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    hailstones: Vec<Hailstone>,
    lower_limit: f64,
    upper_limit: f64,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 24)
    }

    fn parse(&mut self) {
        self.hailstones = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.into())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.hailstones
                .iter()
                .combinations(2)
                .filter(|v| v[0].intersect_xy(v[1], self.lower_limit, self.upper_limit))
                .count(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug, Default)]
struct Hailstone {
    initial_pos: Point3D<f64>,
    velocity: Point3D<f64>,
}

impl Hailstone {
    fn intersect_xy(&self, other: &Self, lower: f64, upper: f64) -> bool {
        let next_self = self.initial_pos + self.velocity;
        let next_other = other.initial_pos + other.velocity;
        let slope_self = (next_self.1 - self.initial_pos.1) / (next_self.0 - self.initial_pos.0);
        let slope_other =
            (next_other.1 - other.initial_pos.1) / (next_other.0 - other.initial_pos.0);
        if approx_equal(slope_self, slope_other) {
            return false;
        }
        let b_self = slope_self * -self.initial_pos.0 + self.initial_pos.1;
        let b_other = slope_other * -other.initial_pos.0 + other.initial_pos.1;
        let x_intersect = (b_other - b_self) / (slope_self - slope_other);
        let y_interect = slope_self * (x_intersect) + b_self;

        if x_intersect < lower || x_intersect > upper || y_interect < lower || y_interect > upper {
            return false;
        }

        let t_self = (x_intersect - self.initial_pos.0) / self.velocity.0;
        let t_other = (x_intersect - other.initial_pos.0) / other.velocity.0;
        if t_self < 0. || t_other < 0. {
            return false;
        }

        true
    }
}

fn approx_equal(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() < f64::EPSILON
}

impl<S: AsRef<str>> From<S> for Hailstone {
    fn from(value: S) -> Self {
        let (pos, vel) = value.as_ref().split_once(" @ ").unwrap();
        let initial_pos = pos
            .split(", ")
            .map(|v| v.trim().parse().unwrap())
            .collect::<Vec<_>>();
        let velocity = vel
            .split(", ")
            .map(|v| v.trim().parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            initial_pos: Point3D(initial_pos[0], initial_pos[1], initial_pos[2]),
            velocity: Point3D(velocity[0], velocity[1], velocity[2]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            lower_limit: 7.,
            upper_limit: 27.,
            ..Default::default()
        };
        day.parse();
        let expected = 2;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
