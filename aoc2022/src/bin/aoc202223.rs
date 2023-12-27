use aoc::runner::{output, run_solution, Runner};

pub fn main() {
    let mut day = AocDay{input: "inputs/day23.txt".into(), ..Default::default()};
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 23)
    }

    fn parse(&mut self) {
        // Parse the input
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "";      

    #[test]
    fn test_part1() {
            let mut day = AocDay{input: INPUT.into(), ..Default::default()};
            day.parse();
            let expected = 0;
            let actual = day.part1()[0].parse().unwrap_or_default();
            assert_eq!(expected, actual);
        }

    #[test]
    fn test_part2() {
            let mut day = AocDay{input: INPUT.into(), ..Default::default()};
            day.parse();
            let expected = 0;
            let actual = day.part2()[0].parse().unwrap_or_default();
            assert_eq!(expected, actual);
        }
    }
        