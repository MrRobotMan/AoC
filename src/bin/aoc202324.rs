use aoc::{
    runner::{output, run_solution, Runner},
    Point3D,
};

use itertools::Itertools;

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day24.txt".into(),
        lower_limit: 200_000_000_000_000.,
        upper_limit: 400_000_000_000_000.,
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    hailstones: Vec<Hailstone>,
    lower_limit: f64,
    upper_limit: f64,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 24)
    }

    fn parse(&mut self) {
        self.hailstones = aoc::read_lines(&self.input)
            .iter()
            .map(|l| l.into())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.hailstones
                .iter()
                .combinations(2)
                .filter(|v| v[0].intersect_xy(v[1], self.lower_limit, self.upper_limit))
                .count(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        // Thanks to DeadlyRedCube for the excellent walkthrough on the linear algrebra.
        // https://github.com/DeadlyRedCube/AdventOfCode/blob/main/2023/AOC2023/D24.h
        // Find 3 velocity vectors that aren't parallel.
        let mut indices = [0; 3];
        while self.hailstones[indices[0]].velocity == self.hailstones[indices[1]].velocity {
            indices[1] += 1;
        }
        while self.hailstones[indices[0]].velocity == self.hailstones[indices[2]].velocity
            || self.hailstones[indices[1]].velocity == self.hailstones[indices[2]].velocity
        {
            indices[2] += 1;
        }
        let stones = indices.map(|i| &self.hailstones[i]);
        // for any given stone, A = A0 + Av*t, our ray (P + Qt) should have a matching t value.
        // With three stones, we have nine equations and nine unknowns
        // (t, u, v, Px, Py, Pz, Qx, Qy, Qz), assuming that any solution for three will work for all:
        // t, u, v, are times of intersection with stones A, B, C
        // P is initial position of thrown stone. This is the thing to solve for.
        // Q is velocity of thrown stone.
        // A0x + Avx*t = Px + Qx*t
        // A0y + Avy*t = Py + Qy*t
        // A0z + Avz*t = Pz + Qz*t
        //
        // B0x + Bvx*u = Px + Qx*u
        // B0y + Bvy*u = Py + Qy*u
        // B0z + Bvz*u = Pz + Qz*u
        //
        // C0x + Cvx*v = Px + Qx*v
        // C0y + Cvy*v = Py + Qy*v
        // C0z + Cvz*v = Pz + Qz*v
        let (a0x, a0y, a0z) = (
            stones[0].initial_pos.0 as f64,
            stones[0].initial_pos.1 as f64,
            stones[0].initial_pos.2 as f64,
        );
        let (avx, avy, avz) = (
            stones[0].velocity.0 as f64,
            stones[0].velocity.1 as f64,
            stones[0].velocity.2 as f64,
        );
        let (b0x, b0y, b0z) = (
            stones[1].initial_pos.0 as f64,
            stones[1].initial_pos.1 as f64,
            stones[1].initial_pos.2 as f64,
        );
        let (bvx, bvy, bvz) = (
            stones[1].velocity.0 as f64,
            stones[1].velocity.1 as f64,
            stones[1].velocity.2 as f64,
        );
        let (c0x, c0y, c0z) = (
            stones[2].initial_pos.0 as f64,
            stones[2].initial_pos.1 as f64,
            stones[2].initial_pos.2 as f64,
        );
        let (cvx, cvy, cvz) = (
            stones[2].velocity.0 as f64,
            stones[2].velocity.1 as f64,
            stones[2].velocity.2 as f64,
        );

        // Expand each and make a set of linear equations elimating t, u, v
        // [Avy - Bvy]*Px - [Avx - Bvx]*Py - [A0y - B0y]*Qx + [A0x - B0x]*Qy = (B0y * Bvx - B0x * Bvy) - (A0y * Avx - A0x * Avy)
        // [Avy - Cvy]*Px - [Avx - Cvx]*Py - [A0y - C0y]*Qx + [A0x - C0x]*Qy = (C0y * Cvx - C0x * Cvy) - (A0y * Avx - A0x * Avy)
        // [Avx - Bvx]*Pz - [Avz - Bvz]*Px - [A0x - B0x]*Qz + [A0z - B0z]*Qx = (B0x * Bvz - B0z * Bvx) - (A0x * Avz - A0z * Avx)
        // [Avx - Cvx]*Pz - [Avz - Cvz]*Px - [A0x - C0x]*Qz + [A0z - C0z]*Qx = (C0x * Cvz - C0z * Cvx) - (A0x * Avz - A0z * Avx)
        // [Avz - Bvz]*Py - [Avy - Bvy]*Pz - [A0z - B0z]*Qy + [A0y - B0y]*Qz = (B0z * Bvy - B0y * Bvz) - (A0z * Avy - A0y * Avz)
        // [Avz - Cvz]*Py - [Avy - Cvy]*Pz - [A0z - C0z]*Qy + [A0y - C0y]*Qz = (C0z * Cvy - C0y * Cvz) - (A0z * Avy - A0y * Avz)
        let abvx = avx - bvx;
        let abvy = avy - bvy;
        let abvz = avz - bvz;

        let acvx = avx - cvx;
        let acvy = avy - cvy;
        let acvz = avz - cvz;

        let ab0x = a0x - b0x;
        let ab0y = a0y - b0y;
        let ab0z = a0z - b0z;

        let ac0x = a0x - c0x;
        let ac0y = a0y - c0y;
        let ac0z = a0z - c0z;

        let rhs0 = (b0y * bvx - b0x * bvy) - (a0y * avx - a0x * avy);
        let rhs1 = (c0y * cvx - c0x * cvy) - (a0y * avx - a0x * avy);
        let rhs2 = (b0x * bvz - b0z * bvx) - (a0x * avz - a0z * avx);
        let rhs3 = (c0x * cvz - c0z * cvx) - (a0x * avz - a0z * avx);
        let rhs4 = (b0z * bvy - b0y * bvz) - (a0z * avy - a0y * avz);
        let rhs5 = (c0z * cvy - c0y * cvz) - (a0z * avy - a0y * avz);
        // Solve each for the first P.
        // Px = ([abvx*ac0x - acvx*ab0x]*Qz + [acvx*ab0z - abvx*ac0z]*Qx + [abvx*rhs3 - acvx*rhs2])/(acvx*abvz - abvx*acvz)
        // Py = ([abvy*ac0y - acvy*ab0y]*Qx + [acvy*ab0x - abvy*ac0x]*Qy + [abvy*rhs1 - acvy*rhs0])/(acvy*abvx - abvy*acvx)
        // Pz = ([abvz*ac0z - acvz*ab0z]*Qy + [acvz*ab0y - abvz*ac0y)*Qz + [abvz*rhs5 - acvz*rhs4])/(acvz*abvy - abvz*acvy)
        // Reduce knowns to new variables
        // Px = (Pxz*Qz + Pxx*Qx + Pxc)/Pxd
        // Py = (Pyx*Qx + Pyy*Qy + Pyc)/Pyd
        // Pz = (Pzy*Qy + Pzz*Qz + Pzc)/Pzd
        let pxx = acvx * ab0z - abvx * ac0z;
        let pyy = acvy * ab0x - abvy * ac0x;
        let pzz = acvz * ab0y - abvz * ac0y;

        let pxz = abvx * ac0x - acvx * ab0x;
        let pzy = abvz * ac0z - acvz * ab0z;
        let pyx = abvy * ac0y - acvy * ab0y;

        let pxc = abvx * rhs3 - acvx * rhs2;
        let pyc = abvy * rhs1 - acvy * rhs0;
        let pzc = abvz * rhs5 - acvz * rhs4;

        let pxd = acvx * abvz - abvx * acvz;
        let pyd = acvy * abvx - abvy * acvx;
        let pzd = acvz * abvy - abvz * acvy;

        // Group and rearrange in terms of Q (stone velocity)
        // Reduct knowns to new variables.
        // abvy*[(Pxz*Qz + Pxx*Qx + Pxc)/Pxd] - abvx*[(Pyx*Qx + Pyy*Qy + Pyc)/Pyd] - ab0y*Qx + ab0x*Qy = rhs0
        // abvx*[(Pzy*Qy + Pzz*Qz + Pzc)/Pzd] - abvz*[(Pxz*Qz + Pxx*Qx + Pxc)/Pxd] - ab0x*Qz + ab0z*Qx = rhs2
        // abvz*[(Pyx*Qx + Pyy*Qy + Pyc)/Pyd] - abvy*[(Pzy*Qy + Pzz*Qz + Pzc)/Pzd] - ab0z*Qy + ab0y*Qz = rhs4
        // And rearrange more
        // ([abvy/Pxd]*Pxz)*Qz + ([abvy/Pxd]*Pxx - [abvx/Pyd]*Pyx - ab0y)*Qx + (ab0x - [abvx/Pyd]*Pyy)*Qy
        //   = rhs0 - [abvy/Pxd]*Pxc + [abvx/Pyd]*Pyc
        let qz0 = (abvy / pxd) * pxz;
        let qx0 = (abvy / pxd) * pxx - (abvx / pyd) * pyx - ab0y;
        let qy0 = ab0x - (abvx / pyd) * pyy;
        let r0 = rhs0 - (abvy / pxd) * pxc + (abvx / pyd) * pyc;

        // ([abvx/Pzd]*Pzy)*Qy + ([abvx/Pzd]*Pzz - [abvz/Pxd]*Pxz - ab0x)*Qz + (ab0z - [abvz/Pxd]*Pxx)*Qx
        //   = rhs2 - [abvx/Pzd]*Pzc + [abvz/Pxd]*Pxc
        let qy1 = (abvx / pzd) * pzy;
        let qz1 = (abvx / pzd) * pzz - (abvz / pxd) * pxz - ab0x;
        let qx1 = ab0z - (abvz / pxd) * pxx;
        let r1 = rhs2 - (abvx / pzd) * pzc + (abvz / pxd) * pxc;

        // ([abvz/Pyd]*Pyx)*Qx + ([abvz/Pyd]*Pyy - [abvy/Pzd]*Pzy - ab0z)*Qy + (ab0y - [abvy/Pzd]*Pzz)*Qz
        //   = rhs4 - [abvz/Pyd]*Pyc + [abvy/Pzd]*Pzc
        let qx2 = (abvz / pyd) * pyx;
        let qy2 = (abvz / pyd) * pyy - (abvy / pzd) * pzy - ab0z;
        let qz2 = ab0y - (abvy / pzd) * pzz;
        let r2 = rhs4 - (abvz / pyd) * pyc + (abvy / pzd) * pzc;

        // Finally solve for q
        let qz = ((qx1 * qy0 - qx0 * qy1) * (qx2 * r0 - qx0 * r2)
            - (qx2 * qy0 - qx0 * qy2) * (qx1 * r0 - qx0 * r1))
            / ((qx2 * qy0 - qx0 * qy2) * (qx0 * qz1 - qx1 * qz0)
                - (qx1 * qy0 - qx0 * qy1) * (qx0 * qz2 - qx2 * qz0));

        let qy = ((qx0 * qz1 - qx1 * qz0) * qz + (qx1 * r0 - qx0 * r1)) / (qx1 * qy0 - qx0 * qy1);

        let qx = (r0 - qy0 * qy - qz0 * qz) / qx0;

        // And last but not least, p
        let px = (pxz * qz + pxx * qx + pxc) / pxd;
        let py = (pyx * qx + pyy * qy + pyc) / pyd;
        let pz = (pzy * qy + pzz * qz + pzc) / pzd;

        output((px + py + pz).round())
    }
}

#[derive(Debug, Default)]
struct Hailstone {
    initial_pos: Point3D<i64>,
    velocity: Point3D<i64>,
}

impl Hailstone {
    fn intersect_xy(&self, other: &Self, lower: f64, upper: f64) -> bool {
        let next_self = self.initial_pos + self.velocity;
        let next_other = other.initial_pos + other.velocity;
        let slope_self =
            (next_self.1 - self.initial_pos.1) as f64 / (next_self.0 - self.initial_pos.0) as f64;
        let slope_other = (next_other.1 - other.initial_pos.1) as f64
            / (next_other.0 - other.initial_pos.0) as f64;
        if approx_equal(slope_self, slope_other) {
            return false;
        }
        let b_self = slope_self * -self.initial_pos.0 as f64 + self.initial_pos.1 as f64;
        let b_other = slope_other * -other.initial_pos.0 as f64 + other.initial_pos.1 as f64;
        let x_intersect = (b_other - b_self) / (slope_self - slope_other);
        let y_interect = slope_self * (x_intersect) + b_self;

        if x_intersect < lower || x_intersect > upper || y_interect < lower || y_interect > upper {
            return false;
        }

        let t_self = (x_intersect - self.initial_pos.0 as f64) / self.velocity.0 as f64;
        let t_other = (x_intersect - other.initial_pos.0 as f64) / other.velocity.0 as f64;
        if t_self < 0. || t_other < 0. {
            return false;
        }

        true
    }
}

fn approx_equal(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() < f64::EPSILON
}

impl<S: AsRef<str>> From<S> for Hailstone {
    fn from(value: S) -> Self {
        let (pos, vel) = value.as_ref().split_once(" @ ").unwrap();
        let initial_pos = pos
            .split(", ")
            .map(|v| v.trim().parse().unwrap())
            .collect::<Vec<_>>();
        let velocity = vel
            .split(", ")
            .map(|v| v.trim().parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            initial_pos: Point3D(initial_pos[0], initial_pos[1], initial_pos[2]),
            velocity: Point3D(velocity[0], velocity[1], velocity[2]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            lower_limit: 7.,
            upper_limit: 27.,
            ..Default::default()
        };
        day.parse();
        let expected = 2;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            lower_limit: 7.,
            upper_limit: 27.,
            ..Default::default()
        };
        day.parse();
        let expected = 47;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
