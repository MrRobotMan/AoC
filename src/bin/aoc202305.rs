use core::panic;
use std::collections::HashSet;

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
    seeds: HashSet<u64>,
    seed_soil: HashSet<(u64, u64, u64)>,
    soil_fertilizer: HashSet<(u64, u64, u64)>,
    fertilizer_water: HashSet<(u64, u64, u64)>,
    water_light: HashSet<(u64, u64, u64)>,
    light_temperature: HashSet<(u64, u64, u64)>,
    temperature_humidity: HashSet<(u64, u64, u64)>,
    humidity_location: HashSet<(u64, u64, u64)>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self) {
        let lines = read_lines("inputs/2023/day05.txt");
        self.process_lines(lines);
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.get_lowest())
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl AocDay {
    fn get_index(&self, item: u64, state: &State) -> u64 {
        let table = match state {
            State::Soils => &self.seed_soil,
            State::Fertilizers => &self.soil_fertilizer,
            State::Waters => &self.fertilizer_water,
            State::Lights => &self.water_light,
            State::Temperatures => &self.light_temperature,
            State::Humidities => &self.temperature_humidity,
            State::Locations => &self.humidity_location,
        };
        for val in table.iter() {
            if (val.1..val.1 + val.2).contains(&item) {
                return val.0 + (item - val.1);
            };
        }
        item
    }

    fn get_lowest(&self) -> u64 {
        let mut lowest = u64::MAX;
        for seed in &self.seeds {
            let mut state = State::default();
            let mut index = *seed;
            while state != State::Locations {
                index = self.get_index(index, &state);
                state = state.next();
            }
            lowest = lowest.min(self.get_index(index, &state));
        }
        lowest
    }

    fn process_lines(&mut self, lines: Vec<String>) {
        let mut lines = lines.iter();
        self.seeds = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|n| n.parse::<_>().unwrap())
            .collect::<_>();
        let mut current = State::default();
        for line in lines {
            if let Some(l) = line.strip_suffix(':') {
                current = match l.split_once(' ') {
                    Some(("seed-to-soil", _)) => State::Soils,
                    Some(("soil-to-fertilizer", _)) => State::Fertilizers,
                    Some(("fertilizer-to-water", _)) => State::Waters,
                    Some(("water-to-light", _)) => State::Lights,
                    Some(("light-to-temperature", _)) => State::Temperatures,
                    Some(("temperature-to-humidity", _)) => State::Humidities,
                    Some(("humidity-to-location", _)) => State::Locations,
                    _ => panic!("Unknown key"),
                };
            } else {
                match current {
                    State::Soils => self.seed_soil.insert(get_parts(line)),
                    State::Fertilizers => self.soil_fertilizer.insert(get_parts(line)),
                    State::Waters => self.fertilizer_water.insert(get_parts(line)),
                    State::Lights => self.water_light.insert(get_parts(line)),
                    State::Temperatures => self.light_temperature.insert(get_parts(line)),
                    State::Humidities => self.temperature_humidity.insert(get_parts(line)),
                    State::Locations => self.humidity_location.insert(get_parts(line)),
                };
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
enum State {
    #[default]
    Soils,
    Fertilizers,
    Waters,
    Lights,
    Temperatures,
    Humidities,
    Locations,
}

impl State {
    fn next(self) -> Self {
        match self {
            Self::Soils => Self::Fertilizers,
            Self::Fertilizers => Self::Waters,
            Self::Waters => Self::Lights,
            Self::Lights => Self::Temperatures,
            Self::Temperatures => Self::Humidities,
            Self::Humidities => Self::Locations,
            Self::Locations => Self::Soils,
        }
    }
}

fn get_parts(value: &str) -> (u64, u64, u64) {
    let mut numbers = value.split(' ').map(|c| c.parse::<_>().unwrap());
    (
        numbers.next().unwrap(),
        numbers.next().unwrap(),
        numbers.next().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "seeds: 79 14 55 13
seed-to-soil map:
50 98 2
52 50 48
soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4
water-to-light map:
88 18 7
18 25 70
light-to-temperature map:
45 77 23
81 45 19
68 64 13
temperature-to-humidity map:
0 69 1
1 0 69
humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_conversion() {
        let expected = (50, 98, 2);
        let actual = get_parts("50 98 2");
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_pasrse() {
        let mut actual = AocDay::default();
        actual.process_lines(INPUT.lines().map(str::to_string).collect::<Vec<String>>());
        assert_eq!(actual.seeds, HashSet::from_iter([79, 14, 55, 13]));
        assert!(actual.humidity_location.contains(&(56, 93, 4)));
    }

    #[test]
    fn test_part1() {
        let expected = 35;
        let mut actual = AocDay::default();
        actual.process_lines(INPUT.lines().map(str::to_string).collect::<Vec<String>>());
        assert_eq!(expected, actual.get_lowest())
    }
}
