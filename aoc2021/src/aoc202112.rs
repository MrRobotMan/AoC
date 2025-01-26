use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    rooms: HashMap<Room, Vec<Room>>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn paths(&self, repeats: usize) -> HashSet<Vec<Room>> {
        let mut queue = vec![(Room::Start, vec![Room::Start])];
        let mut paths = HashSet::new();
        while let Some((room, path)) = queue.pop() {
            if matches!(room, Room::End) {
                paths.insert(path);
                continue;
            }
            for next_room in &self.rooms[&room] {
                let mut new_path = path.clone();
                new_path.push(next_room.clone());
                match next_room {
                    Room::Start => (),
                    Room::Small(_) if count_small(&path, next_room, repeats) => (),
                    _ => queue.push((next_room.clone(), new_path)),
                }
            }
        }
        paths
    }
}

fn count_small(path: &[Room], room: &Room, repeats: usize) -> bool {
    if !path.contains(room) {
        return false;
    }
    let mut seen: HashMap<&Room, usize> = HashMap::new();
    for room in path.iter() {
        if matches!(room, Room::Small(_)) {
            let count = seen.entry(room).or_default();
            *count += 1;
            if *count >= repeats {
                return true;
            }
        }
    }
    false
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2021, 12)
    }

    fn parse(&mut self) {
        for pair in read_lines(&self.input) {
            let (left, right) = pair.split_once('-').unwrap();
            let left = left.parse::<Room>().unwrap();
            let right = right.parse::<Room>().unwrap();
            self.rooms
                .entry(left.clone())
                .or_default()
                .push(right.clone());
            self.rooms.entry(right).or_default().push(left);
        }
    }

    fn part1(&mut self) -> String {
        output(self.paths(1).len())
    }

    fn part2(&mut self) -> String {
        output(self.paths(2).len())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Room {
    Start,
    End,
    Large(String),
    Small(String),
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "start" => Self::Start,
            "end" => Self::End,
            val if val == val.to_uppercase() => Self::Large(val.into()),
            val => Self::Small(val.into()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut day = AocDay::new(
            "start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end",
        );
        day.parse();
        let expected = 10;
        let actual = day.paths(1).len();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        let mut day = AocDay::new(
            "start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end",
        );
        day.parse();
        let expected = 36;
        let actual = day.paths(2).len();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_converts() {
        let actual = "DX".parse::<Room>().unwrap();
        assert!(matches!(actual, Room::Large(_)));
    }
}
