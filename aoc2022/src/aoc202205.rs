use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    pub input: String,
    pub layout: Layout,
    pub instructions: Vec<Instruction>,
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
        (2022, 5)
    }

    fn parse(&mut self) {
        let lines = aoc::lines(self.input.clone());
        let (layout, insructions) = lines.split_once("\n\n").unwrap();
        self.layout = Layout::new(layout, 4, 1);
        self.instructions = insructions.lines().map(Instruction::new).collect();
    }

    fn part1(&mut self) -> String {
        let mut layout = self.layout.clone();
        for inst in &self.instructions {
            layout.process_instruction(inst);
        }
        output(
            layout
                .stacks
                .iter()
                .map(|stack| stack.last().unwrap_or(&' '))
                .collect::<String>(),
        )
    }

    fn part2(&mut self) -> String {
        let mut layout = self.layout.clone();
        for inst in &self.instructions {
            layout.process_many(inst);
        }
        output(
            layout
                .stacks
                .iter()
                .map(|stack| stack.last().unwrap_or(&' '))
                .collect::<String>(),
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct Layout {
    pub stacks: Vec<Vec<char>>,
}

impl Layout {
    pub fn new(data: &str, step: usize, start: usize) -> Self {
        let mut rows: Vec<&str> = data.trim_end().split('\n').collect();
        let stack_count = rows
            .pop() // Last row
            .unwrap() // Only would panic if the vec was empty
            .split_ascii_whitespace() // Get all the column numbers
            .last()
            .unwrap() // Only would panic if the vec was empty
            .parse::<usize>()
            .unwrap();
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];
        for row in rows {
            let mut chrs = row.chars();
            chrs.nth(start - 1); // move the iterator to the correct column
            for (stack, chr) in chrs.step_by(step).enumerate() {
                if chr == ' ' {
                    continue;
                }
                stacks[stack].insert(0, chr);
            }
        }

        Self { stacks }
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.qty {
            if let Some(chr) = self.stacks[instruction.from].pop() {
                self.stacks[instruction.to].push(chr);
            }
        }
    }

    fn process_many(&mut self, instruction: &Instruction) {
        let mut gathered = Vec::new();
        for _ in 0..instruction.qty {
            if let Some(chr) = self.stacks[instruction.from].pop() {
                gathered.push(chr);
            }
        }
        gathered.reverse();
        for chr in gathered {
            self.stacks[instruction.to].push(chr);
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub from: usize,
    pub to: usize,
    pub qty: usize,
}

impl Instruction {
    pub fn new(data: &str) -> Self {
        // instruction is in the from "move _ from _ to _"
        let processed: Vec<&str> = data.split_ascii_whitespace().collect();
        let qty = processed[1].parse::<usize>().unwrap();
        let from = processed[3].parse::<usize>().unwrap() - 1;
        let to = processed[5].parse::<usize>().unwrap() - 1;
        Self { from, to, qty }
    }
}
