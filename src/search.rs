use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

pub trait Searcher: Eq + Hash + Clone {
    fn moves(&self) -> Vec<Self>
    where
        Self: Sized;
    fn is_done(&self) -> bool;
}

pub trait Weighted<T: Searcher> {
    fn weight(&self, _node: &T) -> usize {
        1
    }
}

pub fn dfs<T: Searcher>(start: &T) -> Option<Vec<T>> {
    let mut path = HashMap::new();
    let mut to_visit = vec![start.clone()];
    while let Some(node) = to_visit.pop() {
        if path.contains_key(&node) {
            continue;
        }
        if node.is_done() {
            return Some(get_path(path, node));
        }
        for next_move in node.moves() {
            to_visit.push(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

pub fn bfs<T: Searcher>(start: &T) -> Option<Vec<T>> {
    let mut path = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_front(start.clone());
    while let Some(node) = to_visit.pop_front() {
        if node.is_done() {
            return Some(get_path(path, node));
        }
        for next_move in node.moves() {
            if path.contains_key(&next_move) {
                continue;
            }
            to_visit.push_back(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

pub fn dijkstra<T: Searcher, U: Weighted<T>>(start: &T, map: &U) -> Option<Vec<T>> {
    let mut queue: HashSet<T> = HashSet::new();
    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut path: HashMap<T, T> = HashMap::new();
    let mut index: HashSet<T> = HashSet::new();
    let mut target = None;

    index.insert(start.clone());
    queue.insert(start.clone());
    dist.insert(start.clone(), 0);

    while !queue.is_empty() {
        let shortest = queue
            .iter()
            .map(|item| (item, dist.get(item).unwrap()))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0
            .clone();

        if shortest.is_done() {
            // Found target. Let's build the path.
            target = Some(shortest);
            break;
        }

        if !queue.remove(&shortest) {
            panic!("Tried to remove shortest from queue but it was not found.")
        }

        for next_move in shortest.moves() {
            let step = if queue.contains(&next_move) {
                next_move
            } else if index.insert(next_move.clone()) {
                dist.insert(next_move.clone(), usize::MAX);
                queue.insert(next_move.clone());
                next_move
            } else {
                continue;
            };
            let alt = dist[&shortest] + map.weight(&step);
            if alt < dist[&step] {
                dist.insert(step.clone(), alt);
                path.insert(step.clone(), shortest.clone());
            }
        }
    }

    target.map(|node| get_path(path, node))
}

pub fn get_path<T: Searcher>(moves: HashMap<T, T>, end: T) -> Vec<T> {
    let mut found = Vec::new();
    found.push(end);
    while let Some(node) = moves.get(found.last().unwrap()) {
        found.push(node.clone());
    }
    found.reverse();
    found
}
