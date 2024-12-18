use std::{fmt::Display, str::FromStr};

use num::{Integer, Num};

/// N, E, S, W Steps
pub const CARDINALS: [Vec2D<i64>; 4] = [Vec2D(-1, 0), Vec2D(0, 1), Vec2D(1, 0), Vec2D(0, -1)];

/// NE, SE, SW, NW Steps
pub const ORDINALS: [Vec2D<i64>; 4] = [Vec2D(-1, 1), Vec2D(1, 1), Vec2D(1, -1), Vec2D(-1, -1)];

/// N, NE, E, SE, S, SW, W, NW Steps
pub const COMPASS: [Vec2D<i64>; 8] = [
    Vec2D(-1, 0),
    Vec2D(-1, 1),
    Vec2D(0, 1),
    Vec2D(1, 1),
    Vec2D(1, 0),
    Vec2D(1, -1),
    Vec2D(0, -1),
    Vec2D(-1, -1),
];

/// Directions you can move in a grid
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" | "U" => Ok(Self::North),
            "S" | "D" => Ok(Self::South),
            "E" | "R" => Ok(Self::East),
            "W" | "L" => Ok(Self::West),
            d => Err(format!("Unknown direction {d}")),
        }
    }
}

impl Dir {
    pub fn iter() -> impl Iterator<Item = Dir> {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .iter()
            .copied()
    }
    pub fn delta<T: Integer + Copy>(&self, point: &Vec2D<T>) -> Vec2D<T> {
        let adder: T = Num::from_str_radix("1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        match self {
            Dir::North => Vec2D(point.0 - adder, point.1),
            Dir::South => Vec2D(point.0 + adder, point.1),
            Dir::East => Vec2D(point.0, point.1 + adder),
            Dir::West => Vec2D(point.0, point.1 - adder),
        }
    }
    pub fn scale<T: Integer + Copy>(&self, scale: T) -> Vec2D<T> {
        let zero: T = Num::from_str_radix("0", 10).unwrap_or_else(|_| panic!("Can't convert"));
        let one: T = Num::from_str_radix("1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        let neg_one: T = Num::from_str_radix("-1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        match self {
            Dir::North => Vec2D(neg_one * scale, zero),
            Dir::South => Vec2D(one * scale, zero),
            Dir::East => Vec2D(zero, one * scale),
            Dir::West => Vec2D(zero, neg_one * scale),
        }
    }

    /// Provide with directrion of movement in Point<ROW, COL>
    pub fn value<T: Integer + Copy>(&self) -> Vec2D<T> {
        let zero: T = Num::from_str_radix("0", 10).unwrap_or_else(|_| panic!("Can't convert"));
        let one: T = Num::from_str_radix("1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        let neg_one: T = Num::from_str_radix("-1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        match self {
            Dir::North => Vec2D(neg_one, zero),
            Dir::South => Vec2D(one, zero),
            Dir::East => Vec2D(zero, one),
            Dir::West => Vec2D(zero, neg_one),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec2D<T: Num>(pub T, pub T);

impl TryFrom<char> for Vec2D<i64> {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' | 'U' | '^' => Ok(Self(-1, 0)),
            'S' | 'D' | 'v' => Ok(Self(1, 0)),
            'E' | 'R' | '<' => Ok(Self(0, -1)),
            'W' | 'L' | '>' => Ok(Self(0, 1)),
            d => Err(format!("Unknown direction {d}")),
        }
    }
}

impl<T: Num + Copy> FromIterator<T> for Vec2D<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = vec![];
        for i in iter {
            v.push(i)
        }
        Self(v[0], v[1])
    }
}

impl<T: Num + Copy> std::ops::Add for Vec2D<T> {
    type Output = Vec2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T: Num + Copy> std::ops::Add for &Vec2D<T> {
    type Output = Vec2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T: Num + Copy> std::ops::Sub for Vec2D<T> {
    type Output = Vec2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Num + Copy + std::ops::AddAssign> std::ops::AddAssign for Vec2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Num + Copy + std::ops::SubAssign> std::ops::SubAssign for Vec2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T: Num + Display> Display for Vec2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec3D<T: Num>(pub T, pub T, pub T);

impl<T: Num + Copy> Vec3D<T> {
    /// Return the point in the plane normal to the provided axis.
    /// Normal X => (Y, Z),
    /// Normal Y => (X, Z),
    /// Normal Z => (X, Y)
    pub fn planer(&self, normal: Coordinate) -> Vec2D<T> {
        match normal {
            Coordinate::X => Vec2D::<T>(self.1, self.2),
            Coordinate::Y => Vec2D::<T>(self.0, self.2),
            Coordinate::Z => Vec2D::<T>(self.0, self.1),
        }
    }

    /// Scales a point by some value
    pub fn scale(&self, scale: T) -> Self {
        Self(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}

impl<T: Num + Copy> FromIterator<T> for Vec3D<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        if v.len() != 3 {
            panic!("Can only collect length 3 iterators into points.");
        }
        Self(v[0], v[1], v[2])
    }
}

#[derive(Debug, Default)]
pub enum Coordinate {
    X,
    Y,
    #[default]
    Z,
}

impl<T: Num + Copy> std::ops::Add for Vec3D<T> {
    type Output = Vec3D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl<T: Num + Copy> std::ops::Sub for Vec3D<T> {
    type Output = Vec3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Num + Copy + std::ops::AddAssign> std::ops::AddAssign for Vec3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: Num + Copy + std::ops::SubAssign> std::ops::SubAssign for Vec3D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
