use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

pub trait Searcher: Eq + Hash + Clone {
    fn moves(&self) -> Vec<&Self>
    where
        Self: Sized;
    fn is_done(&self) -> bool;
}

pub fn dfs<T: Searcher>(start: &T) -> Option<VecDeque<T>> {
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

pub fn bfs<T: Searcher>(start: &T) -> Option<VecDeque<T>> {
    let mut path = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_front(start.clone());
    while let Some(node) = to_visit.pop_front() {
        if node.is_done() {
            return Some(get_path(path, node));
        }
        for next_move in node.moves() {
            if path.contains_key(next_move) {
                continue;
            }
            to_visit.push_back(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

fn get_path<T: Searcher>(moves: HashMap<T, T>, end: T) -> VecDeque<T> {
    let mut found = VecDeque::new();
    found.push_front(end);
    while let Some(node) = moves.get(found.front().unwrap()) {
        found.push_front(node.clone());
    }
    found
}
