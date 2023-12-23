use aoc::runner::{output, run_solution, Runner};

fn main() {
    let mut day = AocDay {
        input: "inputs/2023/day23.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    trails: Vec<Vec<Tile>>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 23)
    }

    fn parse(&mut self) {
        for line in aoc::read_grid(&self.input) {
            self.trails
                .push(line.iter().map(|c| c.into()).collect::<Vec<_>>());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output("Unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

impl AocDay {
    fn _dump(&self) {
        for line in self.trails.iter() {
            for ch in line.iter() {
                print!(
                    "{}",
                    match ch {
                        Tile::Path => '.',
                        Tile::Forest => '#',
                        Tile::SlopeUp => '^',
                        Tile::SlopeRight => '>',
                        Tile::SlopeLeft => '<',
                        Tile::SlopeDown => 'v',
                    }
                )
            }
            println!();
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Tile {
    #[default]
    Path, // .
    Forest,     // #
    SlopeUp,    // ^
    SlopeRight, // >
    SlopeLeft,  // <
    SlopeDown,  // v
}

impl From<&char> for Tile {
    fn from(value: &char) -> Self {
        match *value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::SlopeUp,
            '>' => Self::SlopeRight,
            '<' => Self::SlopeLeft,
            'v' => Self::SlopeDown,
            _ => unreachable!("Found unknown character"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 94;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
