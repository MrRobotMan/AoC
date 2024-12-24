use core::panic;
use std::collections::HashMap;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    wires: HashMap<String, bool>,
    gates: HashMap<(String, String, String), Gate>,
    z_values: Vec<Option<bool>>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn simulate(&mut self) -> usize {
        while self.z_values.iter().any(|v| v.is_none()) {
            self.process_gates();
        }
        let mut res = 0;
        for val in self.z_values.iter().rev() {
            res <<= 1;
            if let Some(v) = val {
                res += *v as usize;
            }
        }
        res
    }

    fn process_gates(&mut self) {
        for ((input1, input2, out), op) in self.gates.iter() {
            let inp1 = self.wires.get(input1);
            let inp2 = self.wires.get(input2);
            if let (Some(in1), Some(in2)) = (inp1, inp2) {
                let value = op.process(*in1, *in2);
                if let Some(idx) = out.strip_prefix('z') {
                    self.z_values[idx.parse::<usize>().unwrap()] = Some(value);
                }
                self.wires
                    .entry(out.clone())
                    .and_modify(|v| *v = value)
                    .or_insert(value);
            }
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 24)
    }

    fn parse(&mut self) {
        let lines = read_lines(&self.input);
        for line in lines {
            match line.split_once(": ") {
                Some((wire, value)) => {
                    self.wires.insert(wire.into(), matches!(value, "1"));
                }
                None => {
                    let parts = line.split_whitespace().collect::<Vec<_>>();
                    if parts[4].starts_with("z") {
                        self.z_values.push(None);
                    }
                    self.gates.insert(
                        (parts[0].into(), parts[2].into(), parts[4].into()),
                        parts[1].into(),
                    );
                }
            }
        }
    }

    fn part1(&mut self) -> String {
        output(self.simulate())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn process(&self, inp1: bool, inp2: bool) -> bool {
        match self {
            Gate::And => inp1 & inp2,
            Gate::Or => inp1 | inp2,
            Gate::Xor => inp1 ^ inp2,
        }
    }
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("Bad value {value}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        );
        day.parse();
        let expected = 2024;
        let actual = day.simulate();
        assert_eq!(expected, actual);
    }
}
