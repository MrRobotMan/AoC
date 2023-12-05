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
    seeds: HashSet<usize>,
    seed_soil: HashSet<(usize, usize, usize)>,
    soil_fertilizer: HashSet<(usize, usize, usize)>,
    fertilizer_water: HashSet<(usize, usize, usize)>,
    water_light: HashSet<(usize, usize, usize)>,
    light_temperature: HashSet<(usize, usize, usize)>,
    temperature_humidity: HashSet<(usize, usize, usize)>,
    humidity_location: HashSet<(usize, usize, usize)>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self) {
        let lines = read_lines("inputs/2023/day05.txt");
        let mut lines = lines.iter();
        self.seeds = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|n| n.parse::<_>().unwrap())
            .collect::<_>();
        let mut current = State::Seeds;
        for line in lines {
            if let Some(l) = line.strip_suffix(':') {
                current = match l.split_once(' ') {
                    Some(("seeds", _)) => State::Seeds,
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
                    State::Seeds => panic!("Seeds already processed"),
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

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

#[derive(Debug)]
enum State {
    Seeds,
    Soils,
    Fertilizers,
    Waters,
    Lights,
    Temperatures,
    Humidities,
    Locations,
}

fn get_parts(value: &str) -> (usize, usize, usize) {
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
}
