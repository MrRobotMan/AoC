use aoc::{
    read_string_records,
    runner::{output, Runner},
};
use std::collections::HashSet;
use std::collections::VecDeque;

const _MAX_DIST: usize = 1000;
const ROTATIONS: [[[i64; 3]; 3]; 24] = [
    // Rotation matrix, each is a column.
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
];

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    scanners: Vec<Scanner>,
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
        (2021, 19)
    }

    fn parse(&mut self) {
        self.scanners = read_string_records(&self.input)
            .iter()
            .map(|record| record.into())
            .collect();
    }

    fn part1(&mut self) -> String {
        let mut target = self.scanners[0].clone();
        let mut queue = self
            .scanners
            .iter()
            .skip(1)
            .cloned()
            .collect::<VecDeque<_>>();
        while let Some(scanner) = queue.pop_front() {
            if !target.overlap(&scanner, 12) {
                queue.push_back(scanner);
            };
        }
        output(target.beacons.len())
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

trait Dot<T> {
    type Value;
    fn dot(&self, other: T) -> Self::Value;
}

impl Dot<[i64; 3]> for [i64; 3] {
    type Value = i64;
    fn dot(&self, other: [i64; 3]) -> Self::Value {
        self.iter().zip(other.iter()).fold(0, |r, (a, b)| r + a * b)
    }
}

impl Dot<&[[i64; 3]; 3]> for [i64; 3] {
    type Value = [i64; 3];

    fn dot(&self, other: &[[i64; 3]; 3]) -> Self::Value {
        [self.dot(other[0]), self.dot(other[1]), self.dot(other[2])]
    }
}

trait Delta {
    fn delta(self, other: &Self) -> Self;
}

impl Delta for [i64; 3] {
    fn delta(self, other: &Self) -> Self {
        [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scanner {
    beacons: HashSet<[i64; 3]>,
    index: usize,
}

impl Scanner {
    fn new(beacons: Vec<[i64; 3]>) -> Self {
        Self {
            beacons: HashSet::from_iter(beacons),
            index: 0,
        }
    }

    fn overlap(&mut self, other: &Self, target: usize) -> bool {
        for rotation in other.clone() {
            for node in &rotation.beacons {
                for loc in &self.beacons {
                    let transformed = rotation.translate(loc.delta(node));
                    if self.does_overlap(&transformed, target) {
                        for beacon in &transformed.beacons {
                            self.beacons.insert(*beacon);
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    fn does_overlap(&self, other: &Self, target: usize) -> bool {
        let intersection = self.beacons.intersection(&other.beacons);
        intersection.count() >= target
    }

    fn translate(&self, vector: [i64; 3]) -> Self {
        let mut beacons = HashSet::new();
        for beacon in self.beacons.iter() {
            beacons.insert([
                beacon[0] + vector[0],
                beacon[1] + vector[1],
                beacon[2] + vector[2],
            ]);
        }
        Self {
            beacons,
            index: self.index,
        }
    }
}

impl Iterator for Scanner {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 24 {
            return None;
        }
        let beacons = self
            .beacons
            .iter()
            .map(|b| b.dot(&ROTATIONS[self.index]))
            .collect();
        self.index += 1;
        Some(Self::new(beacons))
    }
}

impl<S: AsRef<str>> From<S> for Scanner {
    fn from(record: S) -> Self {
        Self {
            beacons: record
                .as_ref()
                .lines()
                .skip(1)
                .map(|line| line.beacon())
                .collect(),
            index: 0,
        }
    }
}

trait Beacon {
    fn beacon(self) -> [i64; 3];
}

impl Beacon for &str {
    fn beacon(self) -> [i64; 3] {
        let mut vals = self.split(',');
        [
            vals.next().unwrap().parse().unwrap(),
            vals.next().unwrap().parse().unwrap(),
            vals.next().unwrap().parse().unwrap(),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_scanner() {
        let expected = Scanner::new(vec![[7, -12, 4]]);
        let actual = "--- scanner 42 ---\n7,-12,4".into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rotations() {
        let scanner = Scanner::new(vec![[1, 2, 3]]);
        let mut expected = [
            [1, 2, 3],
            [1, 3, -2],
            [1, -2, -3],
            [1, -3, 2],
            [-1, 2, -3],
            [-1, 3, 2],
            [-1, -2, 3],
            [-1, -3, -2],
            [2, 3, 1],
            [2, 1, -3],
            [2, -3, -1],
            [2, -1, 3],
            [-2, 3, -1],
            [-2, 1, 3],
            [-2, -3, 1],
            [-2, -1, -3],
            [3, 1, 2],
            [3, 2, -1],
            [3, -1, -2],
            [3, -2, 1],
            [-3, 1, -2],
            [-3, 2, 1],
            [-3, -1, 2],
            [-3, -2, -1],
        ];
        expected.sort();
        let mut actual = Vec::new();
        for scan in scanner {
            actual.push(*scan.beacons.iter().next().unwrap());
        }
        actual.sort();
        assert_eq!(expected, *actual);
    }

    #[test]
    fn simple_test() {
        let mut scanner0 = Scanner::new(vec![[0, 2, 0], [4, 1, 0], [3, 3, 0]]);
        let scanner1 = Scanner::new(vec![[0, 2, 0], [4, 1, 0], [3, 3, 0]]);
        let expected = scanner0.clone();
        scanner0.overlap(&scanner1, 2);
        assert_eq!(expected, scanner0);
    }

    #[test]
    fn test_full() {
        let mut scanners = VecDeque::from([
            Scanner::new(vec![
                [404, -588, -901],
                [528, -643, 409],
                [-838, 591, 734],
                [390, -675, -793],
                [-537, -823, -458],
                [-485, -357, 347],
                [-345, -311, 381],
                [-661, -816, -575],
                [-876, 649, 763],
                [-618, -824, -621],
                [553, 345, -567],
                [474, 580, 667],
                [-447, -329, 318],
                [-584, 868, -557],
                [544, -627, -890],
                [564, 392, -477],
                [455, 729, 728],
                [-892, 524, 684],
                [-689, 845, -530],
                [423, -701, 434],
                [7, -33, -71],
                [630, 319, -379],
                [443, 580, 662],
                [-789, 900, -551],
                [459, -707, 401],
            ]),
            Scanner::new(vec![
                [686, 422, 578],
                [605, 423, 415],
                [515, 917, -361],
                [-336, 658, 858],
                [95, 138, 22],
                [-476, 619, 847],
                [-340, -569, -846],
                [567, -361, 727],
                [-460, 603, -452],
                [669, -402, 600],
                [729, 430, 532],
                [-500, -761, 534],
                [-322, 571, 750],
                [-466, -666, -811],
                [-429, -592, 574],
                [-355, 545, -477],
                [703, -491, -529],
                [-328, -685, 520],
                [413, 935, -424],
                [-391, 539, -444],
                [586, -435, 557],
                [-364, -763, -893],
                [807, -499, -711],
                [755, -354, -619],
                [553, 889, -390],
            ]),
            Scanner::new(vec![
                [649, 640, 665],
                [682, -795, 504],
                [-784, 533, -524],
                [-644, 584, -595],
                [-588, -843, 648],
                [-30, 6, 44],
                [-674, 560, 763],
                [500, 723, -460],
                [609, 671, -379],
                [-555, -800, 653],
                [-675, -892, -343],
                [697, -426, -610],
                [578, 704, 681],
                [493, 664, -388],
                [-671, -858, 530],
                [-667, 343, 800],
                [571, -461, -707],
                [-138, -166, 112],
                [-889, 563, -600],
                [646, -828, 498],
                [640, 759, 510],
                [-630, 509, 768],
                [-681, -892, -333],
                [673, -379, -804],
                [-742, -814, -386],
                [577, -820, 562],
            ]),
            Scanner::new(vec![
                [-589, 542, 597],
                [605, -692, 669],
                [-500, 565, -823],
                [-660, 373, 557],
                [-458, -679, -417],
                [-488, 449, 543],
                [-626, 468, -788],
                [338, -750, -386],
                [528, -832, -391],
                [562, -778, 733],
                [-938, -730, 414],
                [543, 643, -506],
                [-524, 371, -870],
                [407, 773, 750],
                [-104, 29, 83],
                [378, -903, -323],
                [-778, -728, 485],
                [426, 699, 580],
                [-438, -605, -362],
                [-469, -447, -387],
                [509, 732, 623],
                [647, 635, -688],
                [-868, -804, 481],
                [614, -800, 639],
                [595, 780, -596],
            ]),
            Scanner::new(vec![
                [727, 592, 562],
                [-293, -554, 779],
                [441, 611, -461],
                [-714, 465, -776],
                [-743, 427, -804],
                [-660, -479, -426],
                [832, -632, 460],
                [927, -485, -438],
                [408, 393, -506],
                [466, 436, -512],
                [110, 16, 151],
                [-258, -428, 682],
                [-393, 719, 612],
                [-211, -452, 876],
                [808, -476, -593],
                [-575, 615, 604],
                [-485, 667, 467],
                [-680, 325, -822],
                [-627, -443, -432],
                [872, -547, -609],
                [833, 512, 582],
                [807, 604, 487],
                [839, -516, 451],
                [891, -625, 532],
                [-652, -548, -490],
                [30, -46, -14],
            ]),
        ]);
        let expected = 79;
        let mut target = scanners.pop_front().unwrap();
        while let Some(scanner) = scanners.pop_front() {
            if !target.overlap(&scanner, 12) {
                scanners.push_back(scanner);
            };
        }
        let actual = target.beacons.len();
        assert_eq!(expected, actual);
    }
}
