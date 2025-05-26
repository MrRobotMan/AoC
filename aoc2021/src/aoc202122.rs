use std::{collections::HashSet, str::FromStr};

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    steps: Vec<Step>,
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
        self.steps = read_lines(&self.input)
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> String {
        let search_volume = Cuboid::new([-50, 50], [-50, 50], [-50, 50]);
        let mut voxels: HashSet<[i64; 3]> = HashSet::new();
        for inst in &self.steps {
            let cuboid: Cuboid = inst.into();
            if !cuboid.within(&search_volume) {
                continue;
            }
            match inst.setting {
                Setting::On => {
                    for x in cuboid.x_range[0]..=cuboid.x_range[1] {
                        for y in cuboid.y_range[0]..=cuboid.y_range[1] {
                            for z in cuboid.z_range[0]..=cuboid.z_range[1] {
                                voxels.insert([x, y, z]);
                            }
                        }
                    }
                }
                Setting::Off => {
                    for x in cuboid.x_range[0]..=cuboid.x_range[1] {
                        for y in cuboid.y_range[0]..=cuboid.y_range[1] {
                            for z in cuboid.z_range[0]..=cuboid.z_range[1] {
                                voxels.remove(&[x, y, z]);
                            }
                        }
                    }
                }
            }
        }
        output(voxels.len())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cuboid {
    x_range: [i64; 2],
    y_range: [i64; 2],
    z_range: [i64; 2],
}

impl Cuboid {
    fn new(x_range: [i64; 2], y_range: [i64; 2], z_range: [i64; 2]) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
        }
    }

    fn volume(&self) -> i64 {
        (self.x_range[1] - self.x_range[0] + 1)
            * (self.y_range[1] - self.y_range[0] + 1)
            * (self.z_range[1] - self.z_range[0] + 1)
    }

    fn merge(&self, other: &Cuboid) -> Option<Cuboid> {
        if self.within(other) {
            return Some(*other);
        }
        if other.within(self) {
            return Some(*self);
        }
        match (
            self.x_range == other.x_range,
            self.y_range == other.y_range,
            self.z_range == other.z_range,
        ) {
            (true, true, true) => Some(*self),
            (true, true, false) => Some(Cuboid::new(
                self.x_range,
                self.y_range,
                [
                    self.z_range[0].min(other.z_range[0]),
                    self.z_range[1].max(other.z_range[1]),
                ],
            )),
            (true, false, true) => Some(Cuboid::new(
                self.x_range,
                [
                    self.y_range[0].min(other.y_range[0]),
                    self.y_range[1].max(other.y_range[1]),
                ],
                self.z_range,
            )),
            (false, true, true) => Some(Cuboid::new(
                [
                    self.x_range[0].min(other.x_range[0]),
                    self.x_range[1].max(other.x_range[1]),
                ],
                self.y_range,
                self.z_range,
            )),
            (false, false, true) => todo!(),
            _ => None,
        }
    }

    fn within(&self, other: &Cuboid) -> bool {
        let x_range = other.x_range[0]..=other.x_range[1];
        let y_range = other.y_range[0]..=other.y_range[1];
        let z_range = other.z_range[0]..=other.z_range[1];
        x_range.contains(&self.x_range[0])
            && x_range.contains(&self.x_range[1])
            && y_range.contains(&self.y_range[0])
            && y_range.contains(&self.y_range[1])
            && z_range.contains(&self.z_range[0])
            && z_range.contains(&self.z_range[1])
    }

    fn split(&self, other: &Cuboid) -> Option<Vec<Cuboid>> {
        let mut cur = *self;
        let mut res = vec![];
        if other.x_range[0] > self.x_range[0] && other.x_range[0] <= self.x_range[1] {
            res.push(Self::new(
                [cur.x_range[0], other.x_range[0] - 1],
                cur.y_range,
                cur.z_range,
            ));
            cur.x_range[0] = other.x_range[0];
        }
        if other.x_range[1] < self.x_range[1] && other.x_range[1] >= self.x_range[0] {
            res.push(Self::new(
                [other.x_range[1] + 1, cur.x_range[1]],
                cur.y_range,
                cur.z_range,
            ));
            cur.x_range[1] = other.x_range[1];
        }
        if other.y_range[0] > self.y_range[0] && other.y_range[0] <= self.y_range[1] {
            res.push(Self::new(
                cur.x_range,
                [cur.y_range[0], other.y_range[0] - 1],
                cur.z_range,
            ));
            cur.y_range[0] = other.y_range[0];
        }
        if other.y_range[1] < self.y_range[1] && other.y_range[1] >= self.y_range[0] {
            res.push(Self::new(
                cur.x_range,
                [other.y_range[1] + 1, cur.y_range[1]],
                cur.z_range,
            ));
            cur.y_range[1] = other.y_range[1];
        }
        if other.z_range[0] > self.z_range[0] && other.z_range[0] <= self.z_range[1] {
            res.push(Self::new(
                cur.x_range,
                cur.y_range,
                [cur.z_range[0], other.z_range[0] - 1],
            ));
            cur.z_range[0] = other.z_range[0];
        }
        if other.z_range[1] < self.z_range[1] && other.z_range[1] >= self.z_range[0] {
            res.push(Self::new(
                cur.x_range,
                cur.y_range,
                [other.z_range[1] + 1, cur.z_range[1]],
            ));
            cur.z_range[1] = other.z_range[1];
        }
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

impl From<&Step> for Cuboid {
    fn from(inst: &Step) -> Self {
        Self {
            x_range: inst.x,
            y_range: inst.y,
            z_range: inst.z,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    setting: Setting,
    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Setting {
    On,
    Off,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (setting, rest) = s.split_once(' ').unwrap();
        let setting = match setting.trim() {
            "on" => Setting::On,
            "off" => Setting::Off,
            _ => unreachable!(),
        };
        let mut coords = rest.split(',').map(|c| {
            let (left, right) = c[2..].split_once("..").unwrap();
            [left.parse().unwrap(), right.parse().unwrap()]
        });
        Ok(Self {
            setting,
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
        let expected = Step {
            setting: Setting::On,
            x: [10, 12],
            y: [-10, 12],
            z: [10, 12],
        };
        let actual = "on x=10..12,y=-10..12,z=10..12".parse().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cuboid_contains() {
        let c1 = Cuboid::new([10, 12], [10, 12], [10, 12]);
        let c2 = Cuboid::new([11, 12], [11, 12], [11, 12]);
        assert!(c2.within(&c1))
    }

    #[test]
    fn test_split() {
        let c1 = Cuboid::new([0, 10], [0, 10], [0, 10]);
        let c2 = Cuboid::new([3, 6], [3, 6], [3, 6]);
        let expected = Some(vec![
            Cuboid::new([0, 2], [0, 10], [0, 10]),
            Cuboid::new([7, 10], [0, 10], [0, 10]),
            Cuboid::new([3, 6], [0, 2], [0, 10]),
            Cuboid::new([3, 6], [7, 10], [0, 10]),
            Cuboid::new([3, 6], [3, 6], [0, 2]),
            Cuboid::new([3, 6], [3, 6], [7, 10]),
        ]);
        let actual = c1.split(&c2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_split() {
        let expected = None;
        let actual = Cuboid::new([0, 6], [5, 20], [100, 140]).split(&Cuboid::new(
            [-5, -1],
            [1, 3],
            [50, 50],
        ));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_merge() {
        let expected = Some(Cuboid::new([0, 5], [11, 20], [10, 30]));
        let actual =
            Cuboid::new([0, 5], [11, 20], [10, 20]).merge(&Cuboid::new([0, 5], [11, 20], [15, 30]));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vol() {
        let expected = 27;
        let actual = Cuboid::new([3, 5], [10, 12], [-7, -5]).volume();
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
