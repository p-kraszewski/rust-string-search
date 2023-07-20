use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Result},
    rc::Rc,
};

pub struct SetMatch {
    root: HashSet<String>,
}

impl SetMatch {
    pub fn load(name: &str) -> Result<Self> {
        let file = File::open(name)?;
        let buf = BufReader::new(file);

        let mut root = HashSet::new();

        for line in buf.lines() {
            let raw_line = line?;

            root.insert(raw_line);
        }
        Ok(Self { root })
    }

    pub fn contains(&self, value: &str) -> bool { self.root.contains(value) }
}
