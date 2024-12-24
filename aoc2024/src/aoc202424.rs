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
    gates: Vec<(String, String, String, Gate)>, //in1, in2, out, op
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
        let mut waiting_for_input = vec![];

        // First pass through. Process what we can.
        for (input1, input2, out, op) in &self.gates {
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
            } else {
                waiting_for_input.push((input1, input2, out, op));
            }
        }
        // Wait for input to exist, then process.
        while !waiting_for_input.is_empty() {
            let mut temp = vec![];
            for (input1, input2, out, op) in waiting_for_input {
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
                } else {
                    temp.push((input1, input2, out, op));
                }
            }
            waiting_for_input = temp;
        }
        value(&self.z_values.iter().filter_map(|v| *v).collect::<Vec<_>>())
    }

    fn find_swapped_gates(&self) -> Vec<String> {
        // Half adder (x0, y0) and full adders chained send output to z.
        // Adder output => z, Adder Carry to c except last carry should go to z.
        //
        // Half adder
        // x00 XOR y00 => z00
        // x00 AND y00 => c00
        //
        // Full adder
        // xn XOR yn => tempn
        // tempn XOR cn-1 => zn
        // xn AND yn => temp2n
        // tempn AND cn-1 => temp3n
        // temp3n OR temp2n => cn

        let mut mismatched = vec![];
        // Find how wires chain together.
        let mut wire_map = HashMap::new();
        for (in1, in2, out, op) in &self.gates {
            wire_map.entry(in1).or_insert(vec![]).push((op, out));
            wire_map.entry(in2).or_insert(vec![]).push((op, out));
        }
        for (in1, in2, out, op) in &self.gates {
            let chain = wire_map.get(out);
            let contains_xor = chain_contains_op(&chain, Gate::Xor);
            let contains_or = chain_contains_op(&chain, Gate::Or);
            let contains_and = chain_contains_op(&chain, Gate::And);
            // Handle initial half adder
            let is_half_adder = in1.ends_with("00") && in2.ends_with("00");

            let is_input_bit = (in1.starts_with('x') && in2.starts_with('y'))
                || (in1.starts_with('y') && in2.starts_with('x'));
            let is_result_bit = out.starts_with('z');
            let last_bit = *out == format!("z{}", self.z_values.len() - 1);
            let is_valid = match op {
                Gate::And => {
                    // Half adder carry bit
                    // Or feeds into a carry bit.
                    is_half_adder || contains_or
                }
                Gate::Or => {
                    // Carries from full adders.
                    // Outputs last bit
                    // Carry bit is (prev carry AND inp1 XOR inp2) OR (inp1 AND inp2)
                    last_bit || (contains_and && contains_xor)
                }
                Gate::Xor => {
                    // Half adder should output the least significant bit.
                    // If a full adder outputs a bit the inputs should be from other ops.
                    // x and y inputs must be xor'd again.
                    (is_half_adder && out == "z00")
                        || (!is_input_bit && is_result_bit)
                        || (is_input_bit && contains_xor)
                }
            };
            if !is_valid {
                mismatched.push(out.clone());
            }
        }
        mismatched.sort();
        mismatched
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
                    let value = matches!(value, "1");
                    self.wires.insert(wire.into(), value);
                }
                None => {
                    let parts = line.split_whitespace().collect::<Vec<_>>();
                    if parts[4].starts_with("z") {
                        self.z_values.push(None);
                    }
                    self.gates.push((
                        parts[0].into(),
                        parts[2].into(),
                        parts[4].into(),
                        parts[1].into(),
                    ));
                }
            }
        }
    }

    fn part1(&mut self) -> String {
        output(self.simulate())
    }

    fn part2(&mut self) -> String {
        output(self.find_swapped_gates().join(","))
    }
}

fn chain_contains_op(chain: &Option<&Vec<(&Gate, &String)>>, gate: Gate) -> bool {
    chain.is_some_and(|v| v.iter().any(|(&g, _)| g == gate))
}

fn value(wires: &[bool]) -> usize {
    let mut res = 0;
    for (idx, wire) in wires.iter().enumerate() {
        res |= (*wire as usize) << idx
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    #[test]
    fn test_create_number() {
        let expected = 4;
        let actual = value(&[false, false, true]);
        assert_eq!(expected, actual);
    }
}
