use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    packet: Packet,
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
        (2021, 16)
    }

    fn parse(&mut self) {
        let binary = from_hex(std::mem::take(&mut read_lines(&self.input)[0]));
        self.packet = Packet::new(&binary.chars().collect::<Vec<_>>());
    }

    fn part1(&mut self) -> String {
        output(self.packet.version_sum())
    }

    fn part2(&mut self) -> String {
        output(self.packet.eval())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Packet {
    version: usize,
    type_id: TypeID,
    op_type: OpType,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new(binary: &[char]) -> Self {
        let version = from_binary(&binary[..3]);
        let type_id = binary[3..].into();
        let op_type = binary[3..].into();
        let mut sub_packets = vec![];
        match op_type {
            OpType::Literal(_) => (),
            OpType::Length(mut l) => {
                let mut start = 22; // 3 version, 3 type, 1 operator, 15 length
                while l > 0 {
                    let packet = Packet::new(&binary[start..]);
                    l -= packet.length();
                    start += packet.length();
                    sub_packets.push(packet);
                }
            }
            OpType::Count(c) => {
                let mut start = 18; // 3 version, 3 type, 1 operator, 11 count
                for _ in 0..c {
                    let packet = Packet::new(&binary[start..]);
                    start += packet.length();
                    sub_packets.push(packet);
                }
            }
        }
        Self {
            version,
            type_id,
            op_type,
            sub_packets,
        }
    }

    fn version_sum(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(Packet::version_sum)
                .sum::<usize>()
    }

    fn length(&self) -> usize {
        match self.op_type {
            OpType::Literal(l) => l,
            OpType::Length(l) => 22 + l,
            OpType::Count(_) => self.sub_packets.iter().fold(18, |acc, p| acc + p.length()),
        }
    }

    fn eval(&self) -> usize {
        match self.type_id {
            TypeID::Literal(v) => v,
            TypeID::Sum => self.sub_packets.iter().fold(0, |acc, p| acc + p.eval()),
            TypeID::Product => self.sub_packets.iter().fold(1, |acc, p| acc * p.eval()),
            TypeID::Min => self.sub_packets.iter().map(Packet::eval).min().unwrap(),
            TypeID::Max => self.sub_packets.iter().map(Packet::eval).max().unwrap(),
            TypeID::Greater => (self.sub_packets[0].eval() > self.sub_packets[1].eval()) as usize,
            TypeID::Less => (self.sub_packets[0].eval() < self.sub_packets[1].eval()) as usize,
            TypeID::Equal => (self.sub_packets[0].eval() == self.sub_packets[1].eval()) as usize,
        }
    }
}

fn parse_literal_to_end(value: &[char]) -> (usize, usize) {
    let mut binary = vec![];
    for (count, chunk) in value.chunks(5).enumerate() {
        binary.extend_from_slice(&chunk[1..]);
        if chunk[0] == '0' {
            return (from_binary(&binary), (count + 1) * 5 + 6);
        }
    }
    unreachable!("Never reached end of literal")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum TypeID {
    Literal(usize),
    #[default]
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl From<&[char]> for TypeID {
    fn from(value: &[char]) -> Self {
        match from_binary(&value[..3]) {
            4 => Self::Literal(parse_literal_to_end(&value[3..]).0),
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => unreachable!("Bad Type ID {value:?}"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OpType {
    Literal(usize),
    Length(usize),
    Count(usize),
}

impl Default for OpType {
    fn default() -> Self {
        Self::Literal(0)
    }
}

impl From<&[char]> for OpType {
    fn from(value: &[char]) -> Self {
        match (from_binary(&value[..3]), value[3]) {
            (4, _) => Self::Literal(parse_literal_to_end(&value[3..]).1),
            (_, '0') => Self::Length(from_binary(&value[4..19])),
            (_, '1') => Self::Count(from_binary(&value[4..15])),
            _ => unreachable!("Bad Type ID {value:?}"),
        }
    }
}

fn from_binary(value: &[char]) -> usize {
    value
        .iter()
        .fold(0, |acc, v| acc * 2 + (*v as u8 - b'0') as usize)
}

fn from_hex<S: AsRef<str>>(value: S) -> String {
    value
        .as_ref()
        .chars()
        .map(|ch| match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => unreachable!("Bad hex char {ch}"),
        })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let expected = Packet {
            version: 6,
            type_id: TypeID::Literal(2021),
            op_type: OpType::Literal(21),
            sub_packets: vec![],
        };
        let actual = Packet::new(&from_hex("D2FE28").chars().collect::<Vec<_>>());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested() {
        let expected = Packet {
            version: 1,
            type_id: TypeID::Less,
            op_type: OpType::Length(27),
            sub_packets: vec![
                Packet {
                    version: 6,
                    type_id: TypeID::Literal(10),
                    op_type: OpType::Literal(11),
                    sub_packets: vec![],
                },
                Packet {
                    version: 2,
                    type_id: TypeID::Literal(20),
                    op_type: OpType::Literal(16),
                    sub_packets: vec![],
                },
            ],
        };
        let actual = Packet::new(&from_hex("38006F45291200").chars().collect::<Vec<_>>());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested_triple() {
        let expected = Packet {
            version: 7,
            type_id: TypeID::Max,
            op_type: OpType::Count(3),
            sub_packets: vec![
                Packet {
                    version: 2,
                    type_id: TypeID::Literal(1),
                    op_type: OpType::Literal(11),
                    sub_packets: vec![],
                },
                Packet {
                    version: 4,
                    type_id: TypeID::Literal(2),
                    op_type: OpType::Literal(11),
                    sub_packets: vec![],
                },
                Packet {
                    version: 1,
                    type_id: TypeID::Literal(3),
                    op_type: OpType::Literal(11),
                    sub_packets: vec![],
                },
            ],
        };
        let actual = Packet::new(&from_hex("EE00D40C823060").chars().collect::<Vec<_>>());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deep_nested() {
        let expected = 16;
        let packet = Packet::new(&from_hex("8A004A801A8002F478").chars().collect::<Vec<_>>());
        let actual = packet.version_sum();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_binary() {
        let expected = 4;
        let actual = from_binary(&['1', '0', '0']);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_hex() {
        let expected = "110100101111111000101000";
        let actual = from_hex("D2FE28");
        assert_eq!(expected, actual);
    }
}
