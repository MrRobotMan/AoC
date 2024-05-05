use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub races: Vec<(i64, i64)>,
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
        (2023, 6)
    }

    fn parse(&mut self) {
        let lines = read_lines(&self.input)
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
        output(self.races.iter().map(get_best_times).product::<i64>())
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

pub fn get_best_times(race: &(i64, i64)) -> i64 {
    // Instead of brute forcing to get each time and comparing.
    // Solve the quadratic.
    // race_time: race.0
    // distance: race.1
    // distance < time * (race_time - time)
    // 0 < -time^2 + time * race_time - distance
    // For the quadratic a = -1, signs involving a have been flipped.
    let race_time = race.0 as f64;
    let distance = -race.1 as f64;
    let mut root1 = (-race_time + (race_time.powi(2) + 4. * distance).sqrt()) / -2.;
    let root2 = (-race_time - (race_time.powi(2) + 4. * distance).sqrt()) / -2.;
    if root1.fract() == 0. {
        root1 += 1.;
    }
    root2.ceil() as i64 - root1.ceil() as i64
}
