use std::{cell::RefCell, collections::HashMap, rc::Rc};

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    tree: Rc<Dir>,
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
        (2022, 7)
    }

    fn parse(&mut self) {
        self.tree = Rc::new(Dir::new(None));
        let mut cwd = self.tree.clone();
        for line in aoc::read_lines(&self.input) {
            let words = line.split(' ').collect::<Vec<&str>>();
            match (words[0], words[1]) {
                ("$", "ls") => continue,
                ("$", "cd") => {
                    cwd = match words[2] {
                        "/" => self.tree.clone(),
                        ".." => cwd.parent.as_ref().unwrap().clone(),
                        x => cwd.directories.borrow()[x].clone(),
                    }
                }
                ("dir", name) => {
                    cwd.directories
                        .borrow_mut()
                        .insert(name.to_owned(), Rc::new(Dir::new(Some(cwd.clone()))));
                }
                (size, name) => {
                    cwd.files
                        .borrow_mut()
                        .insert(name.to_owned(), size.parse().unwrap());
                }
            }
        }
    }

    fn part1(&mut self) -> String {
        let mut total = 0;
        let mut to_visit = vec![self.tree.clone()];
        while let Some(dir) = to_visit.pop() {
            for d in dir.directories.borrow().values() {
                to_visit.push(d.clone());
            }
            let size = dir.size_of();
            if size < 100000 {
                total += size;
            }
        }
        output(total)
    }

    fn part2(&mut self) -> String {
        let disk = 70_000_000;
        let required = 30_000_000;
        let used = self.tree.size_of();
        let space_needed = required - (disk - used);
        let mut smallest_to_delete = used;
        let mut to_visit = vec![self.tree.clone()];
        while let Some(dir) = to_visit.pop() {
            for d in dir.directories.borrow().values() {
                to_visit.push(d.clone());
            }
            let size = dir.size_of();
            if size > space_needed && size <= smallest_to_delete {
                smallest_to_delete = size;
            }
        }
        output(smallest_to_delete)
    }
}

#[derive(Debug, Default)]
struct Dir {
    parent: Option<Rc<Self>>,
    directories: RefCell<HashMap<String, Rc<Self>>>,
    files: RefCell<HashMap<String, usize>>,
}

impl Dir {
    fn new(parent: Option<Rc<Self>>) -> Self {
        Self {
            parent,
            directories: RefCell::new(HashMap::new()),
            files: RefCell::new(HashMap::new()),
        }
    }

    fn size_of(&self) -> usize {
        let mut size = 0;
        for file in self.files.borrow().values() {
            size += file;
        }
        for dir in self.directories.borrow().values() {
            size += dir.size_of();
        }
        size
    }
}
