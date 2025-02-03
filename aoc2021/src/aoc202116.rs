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
        output("Unsolved")
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Packet {
    version: usize,
    type_id_num: usize,
    type_id: TypeID,
    sub_packets: Vec<Packet>,
    length: usize,
}

impl Packet {
    fn new(binary: &[char]) -> Self {
        let version = from_binary(&binary[..3]);
        let type_id_num = from_binary(&binary[3..6]);
        let type_id = binary[3..].into();
        let mut length;
        let mut sub_packets = vec![];
        match type_id {
            TypeID::Literal((_, len)) => length = len,
            TypeID::Length(mut l) => {
                length = 22; // 3 version, 3 type, 1 operator, 15 length
                while l > 0 {
                    let packet = Packet::new(&binary[length..]);
                    l -= packet.length;
                    length += packet.length;
                    sub_packets.push(packet);
                }
            }
            TypeID::Count(c) => {
                length = 18; // 3 version, 3 type, 1 operator, 11 count
                for _ in 0..c {
                    let packet = Packet::new(&binary[length..]);
                    length += packet.length;
                    sub_packets.push(packet);
                }
            }
        }
        Self {
            version,
            type_id_num,
            type_id,
            sub_packets,
            length,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TypeID {
    Literal((usize, usize)),
    Length(usize),
    Count(usize),
}

impl Default for TypeID {
    fn default() -> Self {
        Self::Literal((0, 0))
    }
}

impl From<&[char]> for TypeID {
    fn from(value: &[char]) -> Self {
        match (from_binary(&value[..3]), value[3]) {
            (4, _) => Self::Literal(parse_literal_to_end(&value[3..])),
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
            type_id_num: 4,
            type_id: TypeID::Literal((2021, 21)),
            sub_packets: vec![],
            length: 21,
        };
        let actual = Packet::new(&from_hex("D2FE28").chars().collect::<Vec<_>>());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested() {
        let expected = Packet {
            version: 1,
            type_id_num: 6,
            type_id: TypeID::Length(27),
            sub_packets: vec![
                Packet {
                    version: 6,
                    type_id_num: 4,
                    type_id: TypeID::Literal((10, 11)),
                    sub_packets: vec![],
                    length: 11,
                },
                Packet {
                    version: 2,
                    type_id_num: 4,
                    type_id: TypeID::Literal((20, 16)),
                    sub_packets: vec![],
                    length: 16,
                },
            ],
            length: 49,
        };
        let actual = Packet::new(&from_hex("38006F45291200").chars().collect::<Vec<_>>());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested_triple() {
        let expected = Packet {
            version: 7,
            type_id_num: 3,
            type_id: TypeID::Count(3),
            sub_packets: vec![
                Packet {
                    version: 2,
                    type_id_num: 4,
                    type_id: TypeID::Literal((1, 11)),
                    sub_packets: vec![],
                    length: 11,
                },
                Packet {
                    version: 4,
                    type_id_num: 4,
                    type_id: TypeID::Literal((2, 11)),
                    sub_packets: vec![],
                    length: 11,
                },
                Packet {
                    version: 1,
                    type_id_num: 4,
                    type_id: TypeID::Literal((3, 11)),
                    sub_packets: vec![],
                    length: 11,
                },
            ],
            length: 51,
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
