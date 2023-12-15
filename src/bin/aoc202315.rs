use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day15.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    initialization: Vec<Vec<char>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 15)
    }

    fn parse(&mut self) {
        self.initialization = aoc::read_line(&self.input)
            .split(|c| c == &',')
            .map(|i| i.to_vec())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.initialization.iter().map(|v| score(v)).sum::<u32>())
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

fn score(chars: &[char]) -> u32 {
    chars.iter().fold(0, |acc, c| (acc + *c as u32) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_parse() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = vec![
            vec!['r', 'n', '=', '1'],
            vec!['c', 'm', '-'],
            vec!['q', 'p', '=', '3'],
            vec!['c', 'm', '=', '2'],
            vec!['q', 'p', '-'],
            vec!['p', 'c', '=', '4'],
            vec!['o', 't', '=', '9'],
            vec!['a', 'b', '=', '5'],
            vec!['p', 'c', '-'],
            vec!['p', 'c', '=', '6'],
            vec!['o', 't', '=', '7'],
        ];
        let actual = day.initialization;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 1320;
        let actual = day.part1()[0].parse::<u32>().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
