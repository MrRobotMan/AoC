use aoc::{
    runner::{output, run_solution, Runner},
    Point3D,
};

pub fn main() {
    let mut day = AocDay {
        input: "inputs/day18.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    cubes: Vec<Point3D<i64>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 18)
    }

    fn parse(&mut self) {
        self.cubes = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.split(',').map(|p| p.parse().unwrap()).collect())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self.cubes.iter().fold(0, |acc, cube| {
            acc + DIR
                .iter()
                .filter(|d| !self.cubes.contains(&(*cube + **d)))
                .count()
        });
        output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

const DIR: [Point3D<i64>; 6] = [
    Point3D(0, 0, 1),
    Point3D(0, 0, -1),
    Point3D(0, 1, 0),
    Point3D(0, -1, 0),
    Point3D(1, 0, 0),
    Point3D(-1, 0, 0),
];

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 64;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 0;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
