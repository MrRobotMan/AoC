use std::collections::{HashMap, HashSet};

use aoc::{
    read_grid,
    runner::{output, Runner},
    Point, CARDINALS,
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    farm: HashMap<Point<i64>, (char, Option<usize>)>,
    rows: i64,
    cols: i64,
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
        (2024, 12)
    }

    fn parse(&mut self) {
        let lines = read_grid(&self.input);
        self.rows = lines.len() as i64;
        self.cols = lines[0].len() as i64;
        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.iter().enumerate() {
                self.farm.insert(Point(r as i64, c as i64), (*ch, None));
            }
        }
    }

    fn part1(&mut self) -> String {
        let mut farm = self.farm.clone();
        let mut regions = vec![];
        for row in 0..self.rows {
            for col in 0..self.cols {
                if farm[&Point(row, col)].1.is_none() {
                    let region = find_region(&farm, Point(row, col));
                    for point in &region {
                        farm.entry(*point).and_modify(|v| v.1 = Some(regions.len()));
                    }
                    regions.push(region);
                }
            }
        }
        output(
            regions
                .iter()
                .fold(0, |acc, r| acc + (r.len() * get_perimeter(r))),
        )
    }

    fn part2(&mut self) -> String {
        let mut farm = self.farm.clone();
        let mut regions = vec![];
        for row in 0..self.rows {
            for col in 0..self.cols {
                if farm[&Point(row, col)].1.is_none() {
                    let region = find_region(&farm, Point(row, col));
                    for point in &region {
                        farm.entry(*point).and_modify(|v| v.1 = Some(regions.len()));
                    }
                    regions.push(region);
                }
            }
        }
        output(regions.iter().fold(0, |acc, r| acc + (r.len() * sides(r))))
    }
}

fn get_perimeter(region: &[Point<i64>]) -> usize {
    let mut edges = 0;
    for point in region.iter() {
        for dir in CARDINALS {
            if !region.contains(&(*point + dir)) {
                edges += 1;
            }
        }
    }
    edges
}

fn sides(region: &[Point<i64>]) -> usize {
    if region.len() == 1 {
        return 4;
    }
    let mut edges = HashSet::new();
    let mut sides = 0;
    for point in region.iter() {
        for dir in CARDINALS {
            if !region.contains(&(*point + dir)) {
                edges.insert((*point, *point + dir));
            }
        }
    }
    while let Some(s) = edges.iter().copied().next() {
        let (mut inside, mut outside) = s;
        let (forward, back) = if inside.0 == outside.0 {
            (CARDINALS[0], CARDINALS[2])
        } else {
            (CARDINALS[1], CARDINALS[3])
        };
        let mut step = (inside + forward, outside + forward);
        while edges.contains(&step) {
            (inside, outside) = step;
            step = (inside + forward, outside + forward);
        }
        sides += 1;
        while edges.remove(&(inside, outside)) {
            (inside, outside) = (inside + back, outside + back)
        }
    }

    sides
}

fn find_region(
    farm: &HashMap<Point<i64>, (char, Option<usize>)>,
    loc: Point<i64>,
) -> Vec<Point<i64>> {
    let ch = farm[&loc].0;
    let mut found = HashSet::new();
    let mut visited = HashSet::new();
    found.insert(loc);
    let mut queue = vec![loc];
    while let Some(loc) = queue.pop() {
        for dir in CARDINALS {
            let loc = loc + dir;
            if let Some(chr) = farm.get(&loc) {
                if chr.0 == ch && visited.insert(loc) {
                    queue.push(loc);
                    found.insert(loc);
                }
            }
        }
    }
    let mut found = found.iter().copied().collect::<Vec<_>>();
    found.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
    found
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_farm() -> HashMap<Point<i64>, (char, Option<usize>)> {
        let grid = [
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ];
        grid.iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.chars()
                    .enumerate()
                    .map(|(c, ch)| (Point(r as i64, c as i64), (ch, None)))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    #[test]
    fn test_region() {
        let expected = vec![
            Point(0, 0),
            Point(0, 1),
            Point(0, 2),
            Point(0, 3),
            Point(1, 0),
            Point(1, 1),
            Point(1, 2),
            Point(1, 3),
            Point(2, 2),
            Point(2, 3),
            Point(2, 4),
            Point(3, 2),
        ];
        let actual = find_region(&get_farm(), Point(0, 0));
        assert_eq!(expected, actual);
        assert_eq!(10, sides(&actual))
    }

    #[test]
    fn test_region2() {
        let expected = vec![Point(4, 7)];
        let actual = find_region(&get_farm(), Point(4, 7));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_perimeter() {
        let expected = 18;
        let actual = get_perimeter(&[
            Point(0, 0),
            Point(0, 1),
            Point(0, 2),
            Point(0, 3),
            Point(1, 0),
            Point(1, 1),
            Point(1, 2),
            Point(1, 3),
            Point(2, 2),
            Point(2, 3),
            Point(2, 4),
            Point(3, 2),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_perimeter2() {
        let expected = 16;
        let actual = get_perimeter(&[
            Point(0, 0),
            Point(0, 1),
            Point(0, 2),
            Point(1, 0),
            Point(1, 2),
            Point(2, 0),
            Point(2, 1),
            Point(2, 2),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example1() {
        let mut day = AocDay {
            farm: get_farm(),
            rows: 10,
            cols: 10,
            ..Default::default()
        };
        assert_eq!("1930", day.part1());
        assert_eq!("1206", day.part2());
    }
}
