use aoc::{
    read_lines,
    runner::{output, run_solution, Runner},
};

fn main() {
    let mut day = AocDay::default();
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    races: Vec<(i64, i64)>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn parse(&mut self) {
        let lines = read_lines("inputs/2023/day06.txt")
            .iter()
            .map(|l| {
                let (_, nums) = l.split_once(':').unwrap();
                nums.trim()
            })
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        self.races = lines[0]
            .iter()
            .zip(lines[1].iter())
            .map(|(t, d)| (*t, *d))
            .collect::<Vec<(i64, i64)>>();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.races.iter().map(get_best_times).product::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut time = String::new();
        let mut dist = String::new();
        for race in &self.races {
            time.push_str(&race.0.to_string());
            dist.push_str(&race.1.to_string());
        }
        let race = (time.parse::<i64>().unwrap(), dist.parse::<i64>().unwrap());
        output(get_best_times(&race))
    }
}

fn get_best_times(race: &(i64, i64)) -> usize {
    (0..race.0)
        .map(|c| c * (race.0 - c))
        .filter(|v| *v > race.1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hold_times() {
        let day = AocDay {
            races: vec![(7, 9), (15, 40), (30, 200)],
        };
        let expected = vec![4, 8, 9];
        let actual = day.races.iter().map(get_best_times).collect::<Vec<usize>>();
        assert_eq!(expected, actual);
    }
}
