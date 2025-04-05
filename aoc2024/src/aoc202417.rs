use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    computer: Computer,
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
        (2024, 17)
    }

    fn parse(&mut self) {
        fn get_num(inp: &str) -> usize {
            inp.split_once(": ").unwrap().1.parse().unwrap()
        }
        let lines = read_lines(&self.input);
        self.computer = Computer::new(
            get_num(&lines[0]),
            get_num(&lines[1]),
            get_num(&lines[2]),
            lines[3]
                .split_once(": ")
                .unwrap()
                .1
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect(),
        );
    }

    fn part1(&mut self) -> String {
        output(
            self.computer
                .run()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }

    fn part2(&mut self) -> String {
        output(self.computer.quine())
    }
}

#[derive(Debug, Clone, Default)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instruction_pointer: usize,
    program: Vec<usize>,
    initial: [usize; 3],
}

impl Computer {
    fn new(register_a: usize, register_b: usize, register_c: usize, program: Vec<usize>) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            program,
            initial: [register_a, register_b, register_c],
            ..Default::default()
        }
    }

    fn run(&mut self) -> Vec<usize> {
        let mut res = vec![];
        while self.instruction_pointer < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let literal = self.program[self.instruction_pointer + 1];
            let combo = self.combo(literal);
            self.instruction_pointer += 2;
            match opcode {
                0 => self.register_a >>= combo,
                1 => self.register_b ^= literal,
                2 => self.register_b = combo % 8,
                3 => {
                    if self.register_a != 0 {
                        self.instruction_pointer = literal;
                    }
                }
                4 => self.register_b ^= self.register_c,
                5 => res.push(combo % 8),
                6 => self.register_b = self.register_a >> combo,
                7 => self.register_c = self.register_a >> combo,
                value => unreachable!("Bad opcode {value}"),
            }
        }
        res
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            v => v,
        }
    }

    fn initialize(&mut self) {
        [self.register_a, self.register_b, self.register_c] = self.initial;
        self.instruction_pointer = 0;
    }

    fn quine(&mut self) -> usize {
        // Prog does stuff 3 bits at a time. Calculate from the last to first.
        let len = self.program.len();
        let mut attempted = vec![0];
        for pos in 0..len {
            let mut temp = vec![];
            for attempt in attempted {
                for idx in 0..8 {
                    self.initialize();
                    let attempt = (attempt << 3) | idx;
                    self.register_a = attempt;
                    let res = self.run();
                    if res == self.program[len - 1 - pos..] {
                        temp.push(attempt);
                    }
                }
            }
            attempted = temp;
        }
        *attempted.iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small1() {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        computer.run();
        let expected = 1;
        let actual = computer.register_b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_small2() {
        let mut computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        let expected = vec![0, 1, 2];
        let actual = computer.run();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_small3() {
        let mut computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        let expected = vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0];
        let actual = computer.run();
        assert_eq!(expected, actual);
        assert_eq!(0, computer.register_a);
    }

    #[test]
    fn test_small4() {
        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
        computer.run();
        let expected = 26;
        let actual = computer.register_b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_small5() {
        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
        computer.run();
        let expected = 44354;
        let actual = computer.register_b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example1() {
        let mut computer = Computer::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        let expected = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];
        let actual = computer.run();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let mut computer = Computer::new(2024, 0, 0, vec![0, 3, 5, 4, 3, 0]);
        assert_eq!(computer.quine(), 117440)
    }
}
