use std::collections::HashSet;

use aoc::runner::{output, run_solution, Runner};
use rand::prelude::*;

const TARGET: usize = 3; // Target number of edges to cut.

fn main() {
    let mut day = AocDay {
        input: "inputs/day25.txt".into(),
        ..Default::default()
    };
    run_solution(&mut day);
}

#[derive(Default)]
struct AocDay {
    input: String,
    vertices: HashSet<String>,
    edges: Vec<(String, String)>,
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2023, 25)
    }

    fn parse(&mut self) {
        for line in aoc::read_lines(&self.input) {
            let (part, connected) = line.split_once(": ").unwrap();
            self.vertices.insert(part.into());
            for conn in connected.split_ascii_whitespace() {
                self.vertices.insert(conn.into());
                self.edges.push((part.into(), conn.into()));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut rng = rand::thread_rng();
        loop {
            let mut vertices = self.vertices.clone();
            let mut edges = self.edges.clone();

            while vertices.len() > 2 {
                let idx = rng.gen_range(0..edges.len());
                // Contract the graph
                let (node1, node2) = edges.swap_remove(idx);
                vertices.remove(&node1);
                vertices.remove(&node2);

                // Merge removed nodes into one.
                let new_node = format!("{node1}-{node2}");
                vertices.insert(new_node.clone());

                for (n1, n2) in edges.iter_mut() {
                    if *n1 == node1 || *n1 == node2 {
                        *n1 = new_node.clone();
                    }
                    if *n2 == node1 || *n2 == node2 {
                        *n2 = new_node.clone();
                    }
                }

                // Remove loops
                let mut j = 0;
                while j < edges.len() {
                    let (n1, n2) = &edges[j];
                    if n1 == n2 {
                        edges.swap_remove(j);
                    } else {
                        j += 1;
                    }
                }
            }

            // Check for condition
            if edges.len() == TARGET {
                // As nodes collapse each vertex will contain a list of all nodes
                // collapsed into it. The algorithm will collapse everything into
                // two nodes. Each node will have a - separated list of all nodes
                // it contains. Count these up and multply them.
                return output(
                    vertices
                        .iter()
                        .map(|s| s.split('-').count())
                        .product::<usize>(),
                );
            }
        }
    }

    fn part2(&mut self) -> Vec<String> {
        output("Unsolved")
    }
}

// #[derive(Debug, Default)]
// struct Components {
//     nodes: Vec<String>,
//     neighbors: HashMap<String, Vec<Rc<Edge>>>,
// }

// impl Components {
//     fn is_connected(&self) -> bool {
//         let mut idx = self.nodes[0].as_ref();

//         let mut stack = vec![idx];
//         let mut used = HashSet::from([idx]);

//         while !stack.is_empty() {
//             idx = stack.pop().unwrap();

//             let neighbors = match self.neighbors.get(idx) {
//                 None => continue,
//                 Some(n) => n,
//             };

//             for edge in neighbors {
//                 let adj_node = edge.get_adjacent(idx);
//                 if !used.contains(adj_node) {
//                     used.insert(adj_node);
//                     stack.push(adj_node);
//                 }
//             }
//         }

//         used.len() == self.nodes.len()
//     }
// }

// #[derive(Debug, Default)]
// struct Edge {
//     start: String,
//     end: String,
// }

// impl Edge {
//     fn get_adjacent(&self, base: &str) -> &str {
//         if base == self.start {
//             &self.end
//         } else {
//             &self.start
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 54;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 0;
        let actual = day.part2()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }
}
