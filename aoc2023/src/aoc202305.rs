use core::panic;
use std::collections::HashSet;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default, Clone)]
pub struct AocDay {
    input: String,
    seeds: Vec<i64>,
    seed_soil: HashSet<(i64, i64, i64)>,
    soil_fertilizer: HashSet<(i64, i64, i64)>,
    fertilizer_water: HashSet<(i64, i64, i64)>,
    water_light: HashSet<(i64, i64, i64)>,
    light_temperature: HashSet<(i64, i64, i64)>,
    temperature_humidity: HashSet<(i64, i64, i64)>,
    humidity_location: HashSet<(i64, i64, i64)>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn seeds(&self) -> &[i64] {
        &self.seeds
    }

    #[cfg(test)]
    pub fn humidity_location(&self) -> &HashSet<(i64, i64, i64)> {
        &self.humidity_location
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self) {
        let lines = read_lines(&self.input);
        self.process_lines(lines);
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.get_lowest())
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.get_lowest_rev())
    }
}

impl AocDay {
    fn get_index(&self, item: i64, state: &State) -> i64 {
        let table = match state {
            State::Seeds => return item,
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

    fn get_index_reverse(&self, item: i64, state: &State) -> i64 {
        let table = match state {
            State::Seeds => return item,
            State::Soils => &self.seed_soil,
            State::Fertilizers => &self.soil_fertilizer,
            State::Waters => &self.fertilizer_water,
            State::Lights => &self.water_light,
            State::Temperatures => &self.light_temperature,
            State::Humidities => &self.temperature_humidity,
            State::Locations => &self.humidity_location,
        };
        for val in table.iter() {
            if (val.0..val.0 + val.2).contains(&item) {
                return item + (val.1 - val.0);
            };
        }
        item
    }

    pub fn get_lowest(&self) -> i64 {
        let mut lowest = i64::MAX;
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

    pub fn get_lowest_rev(&self) -> i64 {
        let ranges = self
            .seeds
            .chunks_exact(2)
            .map(|c| c[0]..c[0] + c[1])
            .collect::<Vec<_>>();
        let mut loc = 0;
        loop {
            let mut state = State::Locations;
            let mut index = loc;
            while state != State::Seeds {
                index = self.get_index_reverse(index, &state);
                state = state.prev();
            }

            for range in &ranges {
                if range.contains(&index) {
                    return loc;
                }
            }
            loc += 1;
        }
    }

    pub fn process_lines(&mut self, lines: Vec<String>) {
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
                    _ => panic!("Unknown key"),
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
    Seeds,
}

impl State {
    fn next(self) -> Self {
        match self {
            Self::Seeds => Self::Soils,
            Self::Soils => Self::Fertilizers,
            Self::Fertilizers => Self::Waters,
            Self::Waters => Self::Lights,
            Self::Lights => Self::Temperatures,
            Self::Temperatures => Self::Humidities,
            Self::Humidities => Self::Locations,
            Self::Locations => Self::Soils,
        }
    }
    fn prev(self) -> Self {
        match self {
            Self::Seeds => Self::Locations,
            Self::Soils => Self::Seeds,
            Self::Fertilizers => Self::Soils,
            Self::Waters => Self::Fertilizers,
            Self::Lights => Self::Waters,
            Self::Temperatures => Self::Lights,
            Self::Humidities => Self::Temperatures,
            Self::Locations => Self::Humidities,
        }
    }
}

pub fn get_parts(value: &str) -> (i64, i64, i64) {
    let mut numbers = value.split(' ').map(|c| c.parse::<_>().unwrap());
    (
        numbers.next().unwrap(),
        numbers.next().unwrap(),
        numbers.next().unwrap(),
    )
}
