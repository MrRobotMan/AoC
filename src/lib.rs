use std::{fmt::Debug, fs::read_to_string, path::Path, str::FromStr};

pub mod runner;

/// Read the text of a file to a vec of strings
pub fn read_lines<T: AsRef<Path> + Debug>(path: T) -> Vec<String> {
    read_to_string(path)
        .expect("Failed to open file {path:?}.")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Reads records that are line delineated.
/// For example:
/// 1234
/// 4567
///
/// 3423
/// 2543
pub fn read_number_records<T: AsRef<Path> + Debug, U: FromStr>(path: T) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    read_to_string(path)
        .expect("Failed to open file {path:?}.")
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
pub fn read_numbers<T: AsRef<Path> + Debug, U: FromStr>(path: T) -> Vec<U>
where
    <U as FromStr>::Err: Debug,
{
    read_lines(path)
        .iter()
        .map(|l| l.parse::<U>().expect("Could not parse number {l:?}"))
        .collect()
}

/// Reads the text of a file to a vector of vector of numbers.
pub fn read_number_lists<T: AsRef<Path> + Debug, U: FromStr>(path: T, sep: &str) -> Vec<Vec<U>>
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
pub fn read_line<T: AsRef<Path> + Debug>(path: T) -> Vec<char> {
    read_to_string(path)
        .expect("Failed to open file {path:?}")
        .chars()
        .filter(|&chr| chr != '\n')
        .collect()
}
