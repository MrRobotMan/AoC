use std::collections::{HashMap, HashSet};

use aoc::{
    read_lines,
    runner::{output, Runner},
};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    connections: HashMap<String, HashSet<String>>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    fn find_triples(&self, filter_on: Option<char>) -> Vec<Vec<String>> {
        let mut found = HashSet::new();
        for conn in &self.connections {
            for conn1 in conn.1 {
                for conn2 in &self.connections[conn1] {
                    if self.connections[conn2].contains(conn.0) {
                        let mut conns = [conn.0, conn1, conn2];
                        conns.sort();
                        if filter_on
                            .is_some_and(|ch| !conns.iter().any(|conn| conn.starts_with(ch)))
                        {
                            continue;
                        }
                        found.insert(conns);
                    }
                }
            }
        }
        found
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect()
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2024, 23)
    }

    fn parse(&mut self) {
        let pairs = read_lines(&self.input)
            .iter()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.to_string(), b.to_string())
            })
            .collect::<Vec<_>>();
        for pair in pairs {
            self.connections
                .entry(pair.0.clone())
                .and_modify(|set| {
                    set.insert(pair.1.clone());
                })
                .or_insert(HashSet::from([pair.1.clone()]));
            self.connections
                .entry(pair.1)
                .and_modify(|set| {
                    set.insert(pair.0.clone());
                })
                .or_insert(HashSet::from([pair.0]));
        }
    }

    fn part1(&mut self) -> String {
        output(self.find_triples(Some('t')).len())
    }

    fn part2(&mut self) -> String {
        output(
            bron_kerbosch(
                &self.connections,
                HashSet::new(),
                self.connections.keys().cloned().collect(),
                HashSet::new(),
            )
            .as_slice()
            .join(","),
        )
    }
}

fn bron_kerbosch(
    graph: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
) -> Vec<String> {
    if p.is_empty() && x.is_empty() {
        return r.iter().cloned().collect();
    }
    let u = p.union(&x).next().unwrap();
    let mut res = vec![];
    for vertex in p.clone().difference(&graph[u]) {
        let new = bron_kerbosch(
            graph,
            r.union(&HashSet::from([vertex.clone()])).cloned().collect(),
            p.intersection(&graph[vertex]).cloned().collect(),
            x.intersection(&graph[vertex]).cloned().collect(),
        );
        p.remove(vertex);
        x.insert(vertex.clone());
        if res.len() < new.len() {
            res = new;
        }
    }
    res.sort();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const CONNECTIONS: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_count_triples() {
        let mut day = AocDay::new(CONNECTIONS);
        day.parse();
        let expected = 12;
        let actual = day.find_triples(None).len();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_chiefs_parts() {
        let mut day = AocDay::new(CONNECTIONS);
        day.parse();
        let expected = 7;
        let actual = day.find_triples(Some('t')).len();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay::new(CONNECTIONS);
        day.parse();
        let expected = vec!["co", "de", "ka", "ta"];
        let actual = bron_kerbosch(
            &day.connections,
            HashSet::new(),
            day.connections.keys().cloned().collect(),
            HashSet::new(),
        );
        assert_eq!(expected, actual);
    }
}
