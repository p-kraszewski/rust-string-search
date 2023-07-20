use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
    rc::Rc,
};

struct WordNode {
    is_last:   bool,
    next_word: HashMap<String, WordRef>,
}

pub struct WordMatch {
    root: WordNode,
}

type WordRef = Rc<RefCell<WordNode>>;

impl Default for WordNode {
    fn default() -> Self {
        Self {
            is_last:   false,
            next_word: HashMap::new(),
        }
    }
}

impl WordNode {
    fn insert(&mut self, data: &[&str]) {
        if data.is_empty() {
            self.is_last = true;
        } else {
            let first = data[0];
            let rest = &data[1 ..];
            if !self.next_word.contains_key(first) {
                self.next_word.insert(
                    first.to_string(),
                    Rc::new(RefCell::new(WordNode::default())),
                );
            }
            self.next_word[first].borrow_mut().insert(rest);
        }
    }

    fn contains(&self, data: &[&str]) -> bool {
        if data.is_empty() {
            self.is_last
        } else {
            let first = data[0];
            let rest = &data[1 ..];
            if let Some(next) = self.next_word.get(first) {
                next.borrow().contains(rest)
            } else {
                false
            }
        }
    }
}

impl WordMatch {
    pub fn load(name: &str) -> Result<Self> {
        let file = File::open(name)?;
        let buf = BufReader::new(file);

        let mut root = WordNode::default();

        for line in buf.lines() {
            let raw_line = line?;
            let words = raw_line.split(" ").collect::<Vec<_>>();
            root.insert(&words);
        }
        Ok(Self { root })
    }

    pub fn contains(&self, value: &str) -> bool {
        self.root
            .contains(value.split(" ").collect::<Vec<_>>().as_slice())
    }
}
