use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub initialization: Vec<Step>,
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
        (2023, 15)
    }

    fn parse(&mut self) {
        self.initialization = aoc::read_line(&self.input)
            .split(|c| c == &',')
            .map(Step::new)
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.initialization.iter().map(|s| s.score).sum::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut boxes = vec![LensBox::default(); 256];
        for step in &self.initialization {
            let lens_box = boxes
                .get_mut(hash(&step.label.chars().collect::<Vec<_>>()))
                .unwrap();
            match step.operation {
                '-' => lens_box.remove(&step.label),
                '=' => lens_box.replace(&step.label, step.focal_length),
                c => panic!("Unknown operation {c}"),
            }
        }

        output(
            boxes
                .iter()
                .enumerate()
                .map(|(idx, b)| b.focusing_power(idx + 1))
                .sum::<usize>(),
        )
    }
}

#[derive(Debug, Default, Clone)]
struct LensBox {
    lenses: Vec<(String, usize)>,
}

impl LensBox {
    fn remove(&mut self, label: &str) {
        if let Some(idx) = self.lenses.iter().position(|l| l.0 == label) {
            self.lenses.remove(idx);
        };
    }

    fn replace(&mut self, label: &str, focal_length: usize) {
        if let Some(item) = self.lenses.iter().position(|l| l.0 == label) {
            self.lenses[item].1 = focal_length;
        } else {
            self.lenses.push((label.into(), focal_length));
        }
    }

    fn focusing_power(&self, base: usize) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(slot, (_, focal))| base * (slot + 1) * focal)
            .sum::<usize>()
    }
}

fn hash(text: &[char]) -> usize {
    text.iter().fold(0, |acc, c| (acc + *c as usize) * 17 % 256)
}

#[derive(Debug, PartialEq)]
pub struct Step {
    code: Vec<char>,
    score: usize,
    label: String,
    focal_length: usize,
    operation: char,
}

impl Step {
    pub fn new(chars: &[char]) -> Self {
        let mut sp = chars.split(|c| c == &'-' || c == &'=');
        Self {
            code: chars.to_vec(),
            score: hash(chars),
            label: sp.next().unwrap().iter().collect::<String>(),
            focal_length: sp
                .next()
                .unwrap()
                .iter()
                .fold(0, |acc, c| 10 * acc + *c as usize - '0' as usize),
            operation: if chars.contains(&'-') { '-' } else { '=' },
        }
    }
}
