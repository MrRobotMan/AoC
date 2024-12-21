use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::astar_bag_collect;

use aoc::{
    read_lines,
    runner::{output, Runner},
};

pub struct AocDay {
    pub(crate) input: String,
    codes: Vec<String>,
    num_pad_moves: Paths,
    arrow_pad_moves: Paths,
}

impl Default for AocDay {
    fn default() -> Self {
        Self {
            input: String::new(),
            codes: vec![],
            num_pad_moves: build_paths(
                &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'],
                num_pad_moves,
            ),
            arrow_pad_moves: build_paths(&['<', 'v', '>', '^', 'A'], arrow_pad_moves),
        }
    }
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn presses(&self, code: &str) -> usize {
        let last_robot = get_presses('A', &code.chars().collect::<Vec<_>>(), &self.num_pad_moves);
        let min = last_robot.iter().map(|p| p.len()).min().unwrap();
        let middle_robot = last_robot
            .iter()
            .filter(|p| p.len() == min)
            .flat_map(|r| get_presses('A', r, &self.arrow_pad_moves))
            .collect::<Vec<_>>();
        let min = middle_robot.iter().map(|p| p.len()).min().unwrap();
        let you = middle_robot
            .iter()
            .filter(|p| p.len() == min)
            .flat_map(|r| get_presses('A', r, &self.arrow_pad_moves))
            .collect::<Vec<_>>();
        you.iter().min_by_key(|p| p.len()).unwrap().len()
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 21)
    }

    fn parse(&mut self) {
        self.codes = read_lines(&self.input);
    }

    fn part1(&mut self) -> String {
        output(
            self.codes
                .iter()
                .map(|c| self.presses(c) * get_numeric(c))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn get_numeric(code: &str) -> usize {
    let mut v = 0;
    for char in code.chars() {
        match char as u8 - b'0' {
            c if (0..=9).contains(&c) => v = v * 10 + (c as usize),
            _ => (),
        }
    }
    v
}

type Paths = HashMap<(char, char), Vec<Vec<char>>>;

fn get_presses(cur: char, buttons: &[char], paths: &Paths) -> Vec<Vec<char>> {
    if buttons.is_empty() {
        return vec![vec![]];
    }
    if buttons.len() == 1 {
        return paths[&(cur, buttons[0])]
            .iter()
            .map(|p| {
                let mut temp = p.action_list();
                temp.push('A');
                temp
            })
            .collect();
    }
    let mut res = vec![];
    let mut repeats = 0;
    while buttons[repeats] == cur {
        repeats += 1;
    }
    for path in &paths[&(cur, buttons[repeats])] {
        for p in get_presses(buttons[repeats], &buttons[repeats + 1..], paths) {
            let mut temp = vec!['A'; repeats];
            temp.extend(path.action_list());
            temp.push('A');
            temp.extend(p);
            res.push(temp);
        }
    }
    res
}

fn build_paths(items: &[char], mut successors: impl FnMut(&char) -> Vec<(char, usize)>) -> Paths {
    let mut res = HashMap::new();
    for pair in items.iter().permutations(2) {
        let start = pair[0];
        let end = pair[1];
        res.insert((*start, *end), build_path(start, end, &mut successors));
    }
    res
}

fn build_path(
    start: &char,
    end: &char,
    mut successors: impl FnMut(&char) -> Vec<(char, usize)>,
) -> Vec<Vec<char>> {
    astar_bag_collect(
        start,
        |node| successors(node),
        |_| 1_usize,
        |node| node == end,
    )
    .unwrap()
    .0
}

fn arrow_pad_moves(loc: &char) -> Vec<(char, usize)> {
    match loc {
        '^' => vec![('v', 1), ('A', 1)],
        'A' => vec![('^', 1), ('>', 1)],
        '<' => vec![('v', 1)],
        'v' => vec![('<', 1), ('>', 1), ('^', 1)],
        '>' => vec![('v', 1), ('A', 1)],
        _ => unreachable!(),
    }
}

fn num_pad_moves(loc: &char) -> Vec<(char, usize)> {
    match loc {
        '7' => vec![('8', 1), ('4', 1)],
        '8' => vec![('7', 1), ('9', 1), ('5', 1)],
        '9' => vec![('8', 1), ('6', 1)],
        '4' => vec![('5', 1), ('7', 1), ('1', 1)],
        '5' => vec![('4', 1), ('6', 1), ('8', 1), ('2', 1)],
        '6' => vec![('5', 1), ('9', 1), ('3', 1)],
        '1' => vec![('4', 1), ('2', 1)],
        '2' => vec![('1', 1), ('3', 1), ('5', 1), ('0', 1)],
        '3' => vec![('2', 1), ('6', 1), ('A', 1)],
        '0' => vec![('A', 1), ('2', 1)],
        'A' => vec![('0', 1), ('3', 1)],
        _ => unreachable!(),
    }
}

trait ActionList {
    fn action_list(&self) -> Vec<char>;
}

impl ActionList for Vec<char> {
    fn action_list(&self) -> Vec<char> {
        self.as_slice().windows(2).map(translate_move).collect()
    }
}

fn translate_move(value: &[char]) -> char {
    let rights = [
        ('7', '8'),
        ('8', '9'),
        ('4', '5'),
        ('5', '6'),
        ('1', '2'),
        ('2', '3'),
        ('0', 'A'),
        ('^', 'A'),
        ('<', 'v'),
        ('v', '>'),
    ];
    let lefts = [
        ('8', '7'),
        ('9', '8'),
        ('5', '4'),
        ('6', '5'),
        ('2', '1'),
        ('3', '2'),
        ('A', '0'),
        ('A', '^'),
        ('v', '<'),
        ('>', 'v'),
    ];
    let ups = [
        ('1', '4'),
        ('4', '7'),
        ('0', '2'),
        ('2', '5'),
        ('5', '8'),
        ('A', '3'),
        ('3', '6'),
        ('6', '9'),
        ('v', '^'),
        ('>', 'A'),
    ];
    let downs = [
        ('4', '1'),
        ('7', '4'),
        ('2', '0'),
        ('5', '2'),
        ('8', '5'),
        ('3', 'A'),
        ('6', '3'),
        ('9', '6'),
        ('^', 'v'),
        ('A', '>'),
    ];
    match (value[0], value[1]) {
        right if rights.contains(&right) => '>',
        down if downs.contains(&down) => 'v',
        left if lefts.contains(&left) => '<',
        up if ups.contains(&up) => '^',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_single_path() {
        let expected = [vec!['<', 'v', '>', 'A'], vec!['<', 'v', '^', 'A']];
        let actual = build_path(&'<', &'A', arrow_pad_moves);
        assert!(actual.len() == expected.len());
        assert!(expected.contains(&actual[0]));
        assert!(expected.contains(&actual[1]));
    }

    #[test]
    fn test_num_paths() {
        let expected = 9;
        let actual = &build_paths(
            &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'],
            num_pad_moves,
        )[&('7', 'A')];
        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_keypad_robot() {
        let expected = HashSet::from([
            "<A^A>^^AvvvA".to_string(),
            "<A^A^>^AvvvA".to_string(),
            "<A^A^^>AvvvA".to_string(),
        ]);
        let moves = build_paths(
            &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'],
            num_pad_moves,
        );
        let actual = get_presses('A', &['0', '2', '9', 'A'], &moves)
            .iter()
            .map(|p| p.iter().join(""))
            .collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_numeric() {
        let expected = 29;
        let actual = get_numeric("029A");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_final_presses() {
        let day = AocDay::default();
        let expected = 68;
        let actual = day.presses("029A");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_middle_robot() {
        let expected = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars().collect::<Vec<_>>();
        let moves = build_paths(&['<', 'v', '>', '^', 'A'], arrow_pad_moves);
        let actual = get_presses('A', &"<A^A>^^AvvvA".chars().collect::<Vec<_>>(), &moves);
        assert!(actual.contains(&expected));
    }

    #[test]
    fn test_repeats() {
        let expected = "A>A<vAAA>^A".chars().collect::<Vec<_>>();
        let moves = build_paths(&['<', 'v', '>', '^', 'A'], arrow_pad_moves);
        let actual = get_presses('^', &['^', 'A', 'v', 'v', 'v', 'A'], &moves);
        assert!(actual.contains(&expected));
    }
}
