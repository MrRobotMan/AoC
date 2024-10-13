use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

use aoc::runner::{output, Runner};

#[derive(Default, Debug)]
pub struct AocDay {
    pub input: String,
    pub tunnels: HashMap<String, Valve>,
    pub distances: HashMap<String, HashMap<String, u64>>,
    pub search: Search,
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
        (2022, 16)
    }

    fn parse(&mut self) {
        for line in aoc::read_lines(&self.input).iter() {
            let (flow, connections) = line.split_once(';').unwrap();
            let (valve, rate) = flow.split_once('=').unwrap();
            let rate = rate.parse().unwrap();
            let valve_id = valve.split_ascii_whitespace().nth(1).unwrap();
            let connections = connections
                .split_ascii_whitespace()
                .skip(4)
                .map(|s| s.trim_end_matches(',').to_string())
                .collect::<Vec<_>>();
            self.tunnels.insert(
                valve_id.to_string(),
                Valve {
                    name: valve_id.to_string(),
                    rate,
                    connections,
                },
            );
        }
        for valve in self.tunnels.keys() {
            self.distances
                .insert(valve.clone(), get_distances(valve, &self.tunnels));
        }
        println!("{:?}", self.distances);
    }

    fn part1(&mut self) -> String {
        let path = Path {
            valve: "AA".to_string(),
            turned_on: HashSet::new(),
            time: 30,
            players: 1,
        };
        output(self.search.bfs(&path, &self.tunnels, &self.distances))
    }

    fn part2(&mut self) -> String {
        let path = Path {
            valve: "AA".to_string(),
            turned_on: HashSet::new(),
            time: 26,
            players: 2,
        };
        output(self.search.bfs(&path, &self.tunnels, &self.distances))
    }
}

fn get_distances(start: &str, tunnels: &HashMap<String, Valve>) -> HashMap<String, u64> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();
    seen.insert(start);
    queue.push_front((start, 0));
    while let Some((valve, dist)) = queue.pop_front() {
        let v = &tunnels[valve];
        for path in &v.connections {
            if !seen.insert(path) {
                continue;
            }

            let chamber = &tunnels[path];
            if chamber.rate > 0 && chamber.name != start {
                paths.insert(chamber.name.clone(), dist + 1);
            }

            queue.push_back((&chamber.name, dist + 1));
        }
    }
    paths
}

#[derive(Default, Debug)]
pub struct Search {
    seen: HashMap<Path, u64>,
}

impl Search {
    fn bfs(
        &mut self,
        path: &Path,
        tunnels: &HashMap<String, Valve>,
        distances: &HashMap<String, HashMap<String, u64>>,
    ) -> u64 {
        if let Some(res) = self.seen.get(path) {
            return *res;
        }
        let mut max_flow = if path.players == 2 {
            self.bfs(
                &Path {
                    valve: "AA".into(),
                    turned_on: path.turned_on.clone(),
                    time: 26,
                    players: 1,
                },
                tunnels,
                distances,
            )
        } else {
            0
        };

        if !path.turned_on.contains(&path.valve) && path.time > 0 {
            let mut turned_on = path.turned_on.clone();
            turned_on.insert(path.valve.clone());
            let flow = tunnels[&path.valve].rate * (path.time - 1);
            max_flow = max_flow.max(
                self.bfs(
                    &Path {
                        valve: path.valve.clone(),
                        turned_on,
                        time: path.time - 1,
                        players: path.players,
                    },
                    tunnels,
                    distances,
                ) + flow,
            )
        };

        for (dest, time) in &distances[&path.valve] {
            if *time < path.time {
                max_flow = max_flow.max(self.bfs(
                    &Path {
                        valve: dest.clone(),
                        turned_on: path.turned_on.clone(),
                        time: path.time - *time,
                        players: path.players,
                    },
                    tunnels,
                    distances,
                ))
            }
        }

        self.seen.insert(path.clone(), max_flow);

        max_flow
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Valve {
    name: String,
    rate: u64,
    connections: Vec<String>,
}

#[derive(Default, Debug, Eq, PartialEq, Clone)]
struct Path {
    valve: String,
    turned_on: HashSet<String>,
    time: u64,
    players: u8,
}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.valve.hash(state);
        self.players.hash(state);
        self.time.hash(state);
        let mut v = self.turned_on.iter().collect::<Vec<_>>();
        v.sort();
        for s in v {
            s.hash(state);
        }
    }
}
