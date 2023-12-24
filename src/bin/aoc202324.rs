use aoc::{
    runner::{output, run_solution, Runner},
    Point3D,
};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day24.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    hailstones: Vec<Hailstone>,
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
        output("Unsolved")
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
            ..Default::default()
        };
        day.parse();
        let expected = 2;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
