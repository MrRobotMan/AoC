use std::{
    fmt::{Debug, Display},
    fs::read_to_string,
    path::Path,
    str::FromStr,
};

pub mod runner;
pub mod search;

/// Read the text of a file to a vec of strings
pub fn read_lines<T: AsRef<Path> + Display>(path: T) -> Vec<String> {
    lines(path)
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Gather a string of text or file name to a string
pub fn lines<T: AsRef<Path> + Display>(path: T) -> String {
    match path.as_ref().exists() {
        false => path.to_string(),
        true => read_to_string(path).expect("Failed to open file {path}"),
    }
}
/// Reads records that are line delineated.
/// For example:
/// 1234
/// 4567
///
/// 3423
/// 2543
pub fn read_number_records<T: AsRef<Path> + Display, U: FromStr>(path: T) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    lines(path)
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.lines()
                .filter(|s| s.is_empty())
                .map(|num| num.parse::<U>().expect("Unable to parse number"))
                .collect::<Vec<U>>()
        })
        .collect()
}

/// Reads the text of a file to a vector of numbers.
pub fn read_numbers<T: AsRef<Path> + Display, U: FromStr>(path: T) -> Vec<U>
where
    <U as FromStr>::Err: Debug,
{
    read_lines(path)
        .iter()
        .map(|l| l.parse::<U>().expect("Could not parse number {l:?}"))
        .collect()
}

/// Reads the text of a file to a vector of vector of numbers.
pub fn read_number_lists<T: AsRef<Path> + Display, U: FromStr>(path: T, sep: &str) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    read_lines(path)
        .iter()
        .map(|l| {
            l.split(sep)
                .map(|l| l.parse::<U>().expect("Could not parse number {l:?}"))
                .collect()
        })
        .collect()
}

/// Reads the file to a list of chars.
pub fn read_line<T: AsRef<Path> + Display>(path: T) -> Vec<char> {
    lines(path).chars().filter(|&chr| chr != '\n').collect()
}

/// Reads the file to a grid (vec of vec) of chars
pub fn read_grid<T: AsRef<Path> + Display>(path: T) -> Vec<Vec<char>> {
    lines(path).lines().map(|l| l.chars().collect()).collect()
}

/// Reads the file to grids (vec of vec) of char records line delineated
/// ```
/// let input = "..##.
/// .#...
///
/// ..#..
/// ....#";
/// let expected = vec![
///     vec![
///         vec!['.', '.', '#', '#', '.'],
///         vec!['.', '#', '.', '.', '.']
///         ],
///     vec![
///         vec!['.', '.', '#', '.', '.'],
///         vec!['.', '.', '.', '.', '#']
///         ]
///     ];
/// let actual = aoc::read_grid_records(input);
/// assert_eq!(expected, actual);
/// ```
pub fn read_grid_records<T: AsRef<Path> + Display>(path: T) -> Vec<Vec<Vec<char>>> {
    lines(path)
        .split("\n\n")
        .map(|l| l.lines().map(|r| r.chars().collect()).collect())
        .collect()
}

/// Directions you can move in a grid
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    pub fn delta(&self, point: &(i32, i32)) -> (i32, i32) {
        match self {
            Dir::North => (point.0 - 1, point.1),
            Dir::South => (point.0 + 1, point.1),
            Dir::East => (point.0, point.1 + 1),
            Dir::West => (point.0, point.1 - 1),
        }
    }
}
