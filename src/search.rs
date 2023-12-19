use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

pub trait Graph {
    fn value(&self, row: usize, col: usize) -> usize;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

pub trait Searcher<G: Graph>: Eq + Hash + Clone {
    fn moves(&self, graph: &G) -> Vec<Self>
    where
        Self: Sized;
    fn is_done(&self, graph: &G) -> bool;
}

pub trait Weighted<G: Graph> {
    fn weight(&self, graph: &G) -> usize;
}

pub fn dfs<S: Searcher<G>, G: Graph>(start: &S, graph: &G) -> Option<Vec<S>> {
    let mut path = HashMap::new();
    let mut to_visit = vec![start.clone()];
    while let Some(node) = to_visit.pop() {
        if path.contains_key(&node) {
            continue;
        }
        if node.is_done(graph) {
            return Some(get_path(path, node));
        }
        for next_move in node.moves(graph) {
            to_visit.push(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

pub fn bfs<S: Searcher<G>, G: Graph>(start: &S, graph: &G) -> Option<Vec<S>> {
    let mut path = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_front(start.clone());
    while let Some(node) = to_visit.pop_front() {
        if node.is_done(graph) {
            return Some(get_path(path, node));
        }
        for next_move in node.moves(graph) {
            if path.contains_key(&next_move) {
                continue;
            }
            to_visit.push_back(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

pub fn dijkstra<S: Searcher<G> + Weighted<G>, G: Graph>(start: &S, graph: &G) -> Option<usize> {
    let mut queue: HashSet<S> = HashSet::new();
    let mut dist: HashMap<S, usize> = HashMap::new();
    let mut index: HashSet<S> = HashSet::new();

    queue.insert(start.clone());
    dist.insert(start.clone(), 0);
    index.insert(start.clone());

    while !queue.is_empty() {
        let cur = queue
            .iter()
            .map(|item| (item, dist.get(item).unwrap()))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0
            .clone();

        if cur.is_done(graph) {
            return Some(dist[&cur]);
        }

        if !queue.remove(&cur) {
            panic!("Tried to remove shortest from queue but it was not found.")
        }

        for next_move in cur.moves(graph) {
            let step = if queue.contains(&next_move) {
                next_move
            } else if index.insert(next_move.clone()) {
                // Build the queue as we go instead of putting all nodes in at the start.
                dist.insert(next_move.clone(), usize::MAX);
                queue.insert(next_move.clone());
                next_move
            } else {
                continue;
            };
            let alt = dist[&cur] + step.weight(graph);
            if alt < dist[&step] {
                dist.insert(step.clone(), alt);
            }
        }
    }

    None
}

pub fn a_star<S: Searcher<G> + Weighted<G>, G: Graph, H: Fn(&S) -> usize>(
    start: &S,
    graph: &G,
    heuristic: H,
) -> Option<(Vec<S>, usize)> {
    let mut queue: HashSet<S> = HashSet::new();
    let mut path: HashMap<S, S> = HashMap::new();
    let mut g_score: HashMap<S, usize> = HashMap::new();
    let mut f_score: HashMap<S, usize> = HashMap::new();

    queue.insert(start.clone());
    g_score.insert(start.clone(), 0);
    f_score.insert(start.clone(), heuristic(start));

    while !queue.is_empty() {
        let cur = queue
            .iter()
            .min_by(|a, b| f_score[&a].cmp(&f_score[&b]))
            .unwrap()
            .clone();

        if cur.is_done(graph) {
            return Some((get_path(path, cur.clone()), g_score[&cur]));
        }

        if !queue.remove(&cur) {
            panic!("Tried to remove an item from the queue that was not present.");
        }

        for valid in cur.moves(graph) {
            let tentative = g_score[&cur] + valid.weight(graph);
            if tentative < *g_score.entry(valid.clone()).or_insert(usize::MAX) {
                path.insert(valid.clone(), cur.clone());
                g_score.insert(valid.clone(), tentative);
                f_score.insert(valid.clone(), tentative + heuristic(&valid));
                queue.insert(valid.clone());
            }
        }
    }
    None
}

fn get_path<S: Searcher<G>, G: Graph>(moves: HashMap<S, S>, end: S) -> Vec<S> {
    let mut found = Vec::new();
    found.push(end);
    while let Some(node) = moves.get(found.last().unwrap()) {
        found.push(node.clone());
    }
    found.reverse();
    found
}
