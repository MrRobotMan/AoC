use std::str::FromStr;

use num::{Integer, Num};

/// Directions you can move in a grid
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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
    pub fn delta<T: Integer + Copy>(&self, point: &Point<T>) -> Point<T> {
        let adder: T = Num::from_str_radix("1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        match self {
            Dir::North => Point(point.0 - adder, point.1),
            Dir::South => Point(point.0 + adder, point.1),
            Dir::East => Point(point.0, point.1 + adder),
            Dir::West => Point(point.0, point.1 - adder),
        }
    }
    pub fn scale<T: Integer + Copy>(&self, scale: T) -> (T, T) {
        let zero: T = Num::from_str_radix("0", 10).unwrap_or_else(|_| panic!("Can't convert"));
        let one: T = Num::from_str_radix("1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        let neg_one: T = Num::from_str_radix("-1", 10).unwrap_or_else(|_| panic!("Can't convert"));
        match self {
            Dir::North => (neg_one * scale, zero),
            Dir::South => (one * scale, zero),
            Dir::East => (zero, one * scale),
            Dir::West => (zero, neg_one * scale),
        }
    }

    pub fn value(&self) -> Point<isize> {
        match self {
            Dir::North => Point(-1, 0),
            Dir::South => Point(1, 0),
            Dir::East => Point(0, 1),
            Dir::West => Point(0, -1),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point<T: Num>(pub T, pub T);

impl<T: Num + Copy> std::ops::Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T: Num + Copy> std::ops::Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Num + Copy + std::ops::AddAssign> std::ops::AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Num + Copy + std::ops::SubAssign> std::ops::SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point3D<T: Num>(pub T, pub T, pub T);

impl<T: Num + Copy> Point3D<T> {
    /// Return the point in the plane normal to the provided axis.
    /// Normal X => (Y, Z),
    /// Normal Y => (X, Z),
    /// Normal Z => (X, Y)
    pub fn planer(&self, normal: Coordinate) -> Point<T> {
        match normal {
            Coordinate::X => Point::<T>(self.1, self.2),
            Coordinate::Y => Point::<T>(self.0, self.2),
            Coordinate::Z => Point::<T>(self.0, self.1),
        }
    }

    /// Scales a point by some value
    pub fn scale(&self, scale: T) -> Self {
        Self(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}

impl<T: Num + Copy> FromIterator<T> for Point3D<T> {
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

impl<T: Num + Copy> std::ops::Add for Point3D<T> {
    type Output = Point3D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl<T: Num + Copy> std::ops::Sub for Point3D<T> {
    type Output = Point3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Num + Copy + std::ops::AddAssign> std::ops::AddAssign for Point3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: Num + Copy + std::ops::SubAssign> std::ops::SubAssign for Point3D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
