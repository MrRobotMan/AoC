
fn main() {
    println!("---- 2025: 06 ----");
    let input = puzlib::read_lines("aoc2025/inputs/day06.txt");
    println!("Parsing");
    let model = parse(input);
    println!("Part 1: {}", part1(&model));
    println!("Part 2: {}", part2(&model));
}

fn parse(input: Vec<String>) -> String {
    "unparsed".into()
}

fn part1(_model: &str) -> String {
    "Unsolved".into()
}

fn part2(_model: &str) -> String {
    "Unsolved".into()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let expected = 0;
        let actual = 0;
        assert_eq!(expected, actual);
    }
}
        