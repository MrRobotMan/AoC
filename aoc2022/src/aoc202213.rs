use aoc::runner::{output, Runner};
use std::{cmp::Ordering, fmt::Display, str::Chars};

#[derive(Default)]
pub struct AocDay {
    input: String,
    packets: Vec<(Packet, Packet)>,
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
        (2022, 13)
    }

    fn parse(&mut self) {
        self.packets = aoc::read_string_records(&self.input)
            .iter()
            .map(|pair| {
                let (left, right) = pair.split_once('\n').unwrap();
                (Packet::parse(left), Packet::parse(right))
            })
            .collect();
    }

    fn part1(&mut self) -> String {
        output(self.packets.iter().enumerate().fold(
            0,
            |acc, (idx, (l, r))| {
                if l < r {
                    acc + idx + 1
                } else {
                    acc
                }
            },
        ))
    }

    fn part2(&mut self) -> String {
        let mut packets = Vec::new();
        for (l, r) in self.packets.iter() {
            packets.push(l.clone());
            packets.push(r.clone());
        }
        let sig1 = Packet::parse("[[2]]");
        let sig2 = Packet::parse("[[6]]");
        packets.push(sig1.clone());
        packets.push(sig2.clone());
        packets.sort();
        output(packets.iter().enumerate().fold(1, |acc, (idx, p)| {
            if *p == sig1 || *p == sig2 {
                acc * (idx + 1)
            } else {
                acc
            }
        }))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Self>),
    Int(i32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(l), Packet::List(r)) => {
                for (l, r) in l.iter().zip(r) {
                    let res = l.cmp(r);
                    if res != Ordering::Equal {
                        return res;
                    };
                }
                (l.len()).cmp(&r.len())
            }
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(_), Packet::Int(r)) => self.cmp(&Packet::List(vec![Packet::Int(*r)])),
            (Packet::Int(l), Packet::List(_)) => Packet::List(vec![Packet::Int(*l)]).cmp(other),
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(v) => {
                let mut s = Vec::new();
                s.push("[".to_string());
                for packet in v.iter() {
                    s.push(format!("{}", packet));
                    s.push(",".to_string())
                }
                if s.last().unwrap() == "," {
                    s.pop();
                };
                s.push("]".to_string());
                write!(f, "{}", s.iter().cloned().collect::<String>())?
            }
            Self::Int(i) => write!(f, "{i}")?,
        }
        Ok(())
    }
}

impl Packet {
    fn parse(s: &str) -> Self {
        let mut c = s.chars();
        if c.next().unwrap() != '[' {
            panic!("Bad Input {s}")
        }
        Self::parse_into(&mut c)
    }

    fn parse_into(c: &mut Chars) -> Self {
        let mut res = Vec::new();
        let mut num = -1;
        while let Some(ch) = c.next() {
            match ch {
                '[' => res.push(Self::parse_into(c)),
                ']' => {
                    if num >= 0 {
                        res.push(Self::Int(num))
                    };
                    return Self::List(res);
                }
                ',' => {
                    if num >= 0 {
                        res.push(Self::Int(num));
                        num = -1;
                    };
                }
                '0'..='9' => {
                    if num < 0 {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = num * 10 + (ch as u8 - b'0') as i32;
                    }
                }
                x => panic!("Bad input {x}"),
            }
        }
        Self::List(res)
    }
}
