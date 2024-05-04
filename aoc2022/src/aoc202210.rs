use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    instructions: Vec<Instruction>,
    machine: Machine,
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
        (2022, 10)
    }

    fn parse(&mut self) {
        self.instructions = aoc::read_lines(&self.input)
            .into_iter()
            .map(|line| match line.split_once(' ') {
                None => Instruction::Noop,
                Some((_, v)) => Instruction::Addx(v.parse().unwrap()),
            })
            .collect();
        self.machine = Machine::new(40);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut strength = 0;
        let mut check = 20;
        let step = 40;
        for line in &self.instructions {
            match line {
                Instruction::Noop => {
                    self.machine.noop();
                    if self.machine.cycle >= check {
                        strength += check * self.machine.register;
                        check += step;
                    }
                }
                &Instruction::Addx(v) => {
                    let prev = self.machine.register;
                    self.machine.addx(v);
                    if self.machine.cycle >= check {
                        strength += check * prev;
                        check += step;
                    }
                }
            }
        }
        output(strength)
    }

    fn part2(&mut self) -> Vec<String> {
        output(&self.machine.display)
    }
}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

#[derive(Debug, Default)]
struct Machine {
    cycle: i32,
    register: i32,
    display: String,
    display_size: i32,
}

impl Machine {
    fn new(display_size: i32) -> Self {
        Self {
            cycle: 0,
            register: 1,
            display: "".into(),
            display_size,
        }
    }

    fn cycle(&mut self) {
        if self.cycle % self.display_size == 0 {
            self.display.push('\n')
        }
        let sprite = self.register + (self.cycle / self.display_size) * self.display_size;
        if (self.cycle - sprite).abs() <= 1 {
            self.display.push('#')
        } else {
            self.display.push('.')
        }
        self.cycle += 1;
    }

    fn noop(&mut self) {
        self.cycle();
    }

    fn addx(&mut self, value: i32) {
        self.cycle();
        self.cycle();
        self.register += value;
    }
}
