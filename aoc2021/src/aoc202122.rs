use std::str::FromStr;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    cuboids: Vec<Cuboid>,
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
        (2021, 22)
    }

    fn parse(&mut self) {
        self.cuboids = read_lines(&self.input)
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> String {
        output(get_vol(
            &self.cuboids,
            Some(Cuboid {
                status: Status::On,
                x: [-50, 50],
                y: [-50, 50],
                z: [-50, 50],
            }),
        ))
    }

    fn part2(&mut self) -> String {
        output(get_vol(&self.cuboids, None))
    }
}

fn get_vol(cuboids: &[Cuboid], limits: Option<Cuboid>) -> i64 {
    let mut voxels = vec![];
    for cuboid in cuboids {
        if let Some(limit) = limits {
            if !cuboid.within(&limit) {
                continue;
            }
        }
        let mut to_add = if cuboid.status == Status::On {
            vec![*cuboid]
        } else {
            vec![]
        };
        for voxel in &voxels {
            if let Some(core) = cuboid.intersection(voxel) {
                to_add.push(core);
            }
        }
        voxels.extend(to_add);
    }
    voxels.iter().fold(0, |acc, c| acc + c.volume())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cuboid {
    status: Status,
    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2],
}

impl Cuboid {
    fn volume(&self) -> i64 {
        // Negative when off
        self.status.value()
            * (self.x[1] - self.x[0] + 1)
            * (self.y[1] - self.y[0] + 1)
            * (self.z[1] - self.z[0] + 1)
    }

    fn within(&self, other: &Self) -> bool {
        let x = other.x[0]..=other.x[1];
        let y = other.y[0]..=other.y[1];
        let z = other.z[0]..=other.z[1];
        x.contains(&self.x[0])
            && x.contains(&self.x[1])
            && y.contains(&self.y[0])
            && y.contains(&self.y[1])
            && z.contains(&self.z[0])
            && z.contains(&self.z[1])
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        // Check that overlap actually occurs.
        if self.x[0] > other.x[1]
            || self.x[1] < other.x[0]
            || self.y[0] > other.y[1]
            || self.y[1] < other.y[0]
            || self.z[0] > other.z[1]
            || self.z[1] < other.z[0]
        {
            return None;
        }
        let values = |a: [i64; 2], b: [i64; 2]| [a[0].max(b[0]), a[1].min(b[1])];
        Some(Self {
            status: other.status.flip(),
            x: values(self.x, other.x),
            y: values(self.y, other.y),
            z: values(self.z, other.z),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Status {
    On,
    Off,
}

impl Status {
    fn flip(&self) -> Self {
        match self {
            Status::On => Status::Off,
            Status::Off => Status::On,
        }
    }

    fn value(&self) -> i64 {
        match self {
            Status::On => 1,
            Status::Off => -1,
        }
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (setting, rest) = s.split_once(' ').unwrap();
        let setting = match setting.trim() {
            "on" => Status::On,
            "off" => Status::Off,
            _ => unreachable!(),
        };
        let mut coords = rest.split(',').map(|c| {
            let (left, right) = c[2..].split_once("..").unwrap();
            [left.parse().unwrap(), right.parse().unwrap()]
        });
        Ok(Self {
            status: setting,
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = Cuboid {
            status: Status::On,
            x: [10, 12],
            y: [-10, 12],
            z: [10, 12],
        };
        let actual = "on x=10..12,y=-10..12,z=10..12".parse().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cuboid_contains() {
        let c1 = Cuboid {
            status: Status::On,
            x: [10, 12],
            y: [10, 12],
            z: [10, 12],
        };
        let c2 = Cuboid {
            status: Status::On,
            x: [11, 12],
            y: [11, 12],
            z: [11, 12],
        };
        assert!(c2.within(&c1))
    }

    #[test]
    fn test_int() {
        let c1 = Cuboid {
            status: Status::On,
            x: [0, 10],
            y: [0, 10],
            z: [0, 10],
        };
        let c2 = Cuboid {
            status: Status::On,
            x: [3, 6],
            y: [3, 6],
            z: [3, 6],
        };
        let expected = Some(Cuboid {
            status: Status::Off,
            x: [3, 6],
            y: [3, 6],
            z: [3, 6],
        });
        let actual = c1.intersection(&c2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_int_corner() {
        let c1 = Cuboid {
            status: Status::On,
            x: [0, 10],
            y: [0, 10],
            z: [0, 10],
        };
        let c2 = Cuboid {
            status: Status::Off,
            x: [-3, 1],
            y: [-3, 1],
            z: [-3, 1],
        };
        let expected = Some(Cuboid {
            status: Status::On,
            x: [0, 1],
            y: [0, 1],
            z: [0, 1],
        });
        let actual = c1.intersection(&c2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_int() {
        let expected = None;
        let actual = Cuboid {
            status: Status::On,
            x: [0, 6],
            y: [5, 20],
            z: [100, 140],
        }
        .intersection(&Cuboid {
            status: Status::On,
            x: [-5, -1],
            y: [1, 3],
            z: [50, 50],
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_int2() {
        let expected = None;
        let actual = Cuboid {
            status: Status::On,
            x: [13, 13],
            y: [11, 13],
            z: [11, 13],
        }
        .intersection(&Cuboid {
            status: Status::On,
            x: [9, 11],
            y: [9, 11],
            z: [9, 11],
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vol() {
        let expected = 27;
        let actual = Cuboid {
            status: Status::On,
            x: [3, 5],
            y: [10, 12],
            z: [-7, -5],
        }
        .volume();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example() {
        let mut day = AocDay::new(
            "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682",
        );
        day.parse();
        let expected = 590784;
        let actual = day.part1().parse::<i64>().unwrap();
        assert_eq!(expected, actual);
    }
}
