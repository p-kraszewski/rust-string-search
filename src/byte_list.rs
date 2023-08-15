use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader, Result},
    rc::Rc,
};

struct ByteNode {
    is_last: bool,
    next_letter: [Option<ByteNodeRef>; 256],
}

pub struct ByteMatch {
    root: ByteNode,
}

type ByteNodeRef = Rc<RefCell<ByteNode>>;

impl Default for ByteNode {
    fn default() -> Self {
        Self {
            is_last: false,
            next_letter: std::array::from_fn(|_| None),
        }
    }
}

impl ByteNode {
    fn insert(&mut self, data: &[u8]) {
        if data.is_empty() {
            self.is_last = true;
        } else {
            let first = data[0] as usize;
            let rest = &data[1..];
            if self.next_letter[first].is_none() {
                self.next_letter[first] = Some(Rc::new(RefCell::new(ByteNode::default())));
            }
            self.next_letter[first]
                .as_mut()
                .unwrap()
                .borrow_mut()
                .insert(rest);
        }
    }

    fn contains(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            self.is_last
        } else {
            let first = data[0] as usize;
            let rest = &data[1..];
            if self.next_letter[first].is_none() {
                false
            } else {
                self.next_letter[first]
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .contains(rest)
            }
        }
    }
}

impl ByteMatch {
    pub fn load(name: &str) -> Result<Self> {
        let file = File::open(name)?;
        let buf = BufReader::new(file);

        let mut root = ByteNode::default();

        for line in buf.lines() {
            let raw_line = line?;
            root.insert(raw_line.as_bytes());
        }
        Ok(Self { root })
    }

    pub fn contains(&self, value: &str) -> bool {
        self.root.contains(value.as_bytes())
    }
}
