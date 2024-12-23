use std::{collections::HashMap, sync::LazyLock};

use aoc::{
    read_lines,
    runner::{output, Runner},
};
use itertools::Itertools;
use pathfinding::prelude::astar_bag_collect;

pub struct AocDay {
    pub(crate) input: String,
    codes: Vec<String>,
    num_pad_moves: Paths,
}
impl Default for AocDay {
    fn default() -> Self {
        Self {
            input: String::new(),
            codes: vec![],
            num_pad_moves: build_numpad_paths(
                &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'],
                num_pad_moves,
            ),
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

    fn presses(&self, code: &str, robots: usize) -> usize {
        get_num_presses('A', &code.chars().collect::<Vec<_>>(), &self.num_pad_moves)
            .iter()
            .map(|arrows| get_arrow_presses('A', arrows, robots - 1))
            .min()
            .unwrap()
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
                .map(|c| self.presses(c, 2) * get_numeric(c))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> String {
        output(
            self.codes
                .iter()
                .map(|c| self.presses(c, 25) * get_numeric(c))
                .sum::<usize>(),
        )
    }
}

fn get_numeric(code: &str) -> usize {
    code[..3].parse().unwrap()
}

fn get_arrow_presses(mut cur: char, sequence: &[char], depth: usize) -> usize {
    let mut cache = BEST_ARROW_PATHS // Key = start, end, depth. Value = steps needed.
        .iter()
        .map(|(k, v)| ((k.0, k.1, 0_usize), v.len()))
        .collect::<HashMap<_, _>>();
    let mut res = 0;
    for &button in sequence {
        res += inst_length(cur, button, depth, &mut cache);
        cur = button;
    }
    res
}

fn inst_length(
    start: char,
    target: char,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if let Some(known) = cache.get(&(start, target, depth)) {
        return *known;
    }
    let mut res = 0;
    let mut current = 'A';
    for &button in &BEST_ARROW_PATHS[&(start, target)] {
        res += inst_length(current, button, depth - 1, cache);
        current = button
    }
    cache.insert((start, target, depth), res);
    res
}

type Path = HashMap<(char, char), Vec<char>>;

static BEST_ARROW_PATHS: LazyLock<Path> = LazyLock::new(|| {
    HashMap::from([
        (('A', 'A'), vec!['A']),
        (('A', '^'), vec!['<', 'A']),
        (('A', '>'), vec!['v', 'A']),
        (('A', 'v'), vec!['<', 'v', 'A']),
        (('A', '<'), vec!['v', '<', '<', 'A']),
        (('^', 'A'), vec!['>', 'A']),
        (('^', '^'), vec!['A']),
        (('^', '>'), vec!['v', '>', 'A']),
        (('^', 'v'), vec!['v', 'A']),
        (('^', '<'), vec!['v', '<', 'A']),
        (('v', 'A'), vec!['^', '>', 'A']),
        (('v', '^'), vec!['^', 'A']),
        (('v', '>'), vec!['>', 'A']),
        (('v', 'v'), vec!['A']),
        (('v', '<'), vec!['<', 'A']),
        (('<', 'A'), vec!['>', '>', '^', 'A']),
        (('<', '^'), vec!['>', '^', 'A']),
        (('<', '>'), vec!['>', '>', 'A']),
        (('<', 'v'), vec!['>', 'A']),
        (('<', '<'), vec!['A']),
        (('>', 'A'), vec!['^', 'A']),
        (('>', '^'), vec!['<', '^', 'A']),
        (('>', '>'), vec!['A']),
        (('>', 'v'), vec!['<', 'A']),
        (('>', '<'), vec!['<', '<', 'A']),
    ])
});

type Paths = HashMap<(char, char), Vec<Vec<char>>>;

fn get_num_presses(cur: char, buttons: &[char], paths: &Paths) -> Vec<Vec<char>> {
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
        for p in get_num_presses(buttons[repeats], &buttons[repeats + 1..], paths) {
            let mut temp = vec!['A'; repeats];
            temp.extend(path.action_list());
            temp.push('A');
            temp.extend(p);
            res.push(temp);
        }
    }
    res
}

fn build_numpad_paths(
    items: &[char],
    mut successors: impl FnMut(&char) -> Vec<(char, usize)>,
) -> Paths {
    let mut res = HashMap::new();
    for pair in items.iter().permutations(2) {
        let start = pair[0];
        let end = pair[1];
        res.insert(
            (*start, *end),
            build_numpad_path(start, end, &mut successors),
        );
    }
    res
}

fn build_numpad_path(
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
    ];
    let lefts = [
        ('8', '7'),
        ('9', '8'),
        ('5', '4'),
        ('6', '5'),
        ('2', '1'),
        ('3', '2'),
        ('A', '0'),
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

    #[test]
    fn test_keypad_robot() {
        let expected = "<A^A>^^AvvvA".chars().collect::<Vec<_>>();
        let moves = build_numpad_paths(
            &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'],
            num_pad_moves,
        );
        let actual = get_num_presses('A', &['0', '2', '9', 'A'], &moves);
        assert!(actual.contains(&expected));
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
        let expected = vec![68, 60, 68, 64, 64];
        let inputs = ["029A", "980A", "179A", "456A", "379A"];
        let actual = inputs
            .iter()
            .map(|code| day.presses(code, 2))
            .collect::<Vec<_>>();

        assert_eq!(expected, actual);
        assert_eq!(
            126384,
            actual
                .iter()
                .zip(inputs.iter())
                .fold(0, |acc, (val, inp)| acc + *val * get_numeric(inp))
        )
    }

    #[test]
    fn test_middle_robot() {
        let expected = "v<<A>>^A<A>AvA^<AA>Av<AAA^>A".len();
        let actual = get_arrow_presses('A', &"<A^A>^^AvvvA".chars().collect::<Vec<_>>(), 0);
        assert_eq!(expected, actual);
    }
}
