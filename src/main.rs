use std::io;
use wordList::{byte_list, string_set, word_list};

// mod byte_list;
// mod string_set;
// mod word_list;

const FILE: &str = "src/byte_list.rs";

fn main() -> io::Result<()> {
    let t1 = byte_list::ByteMatch::load(FILE)?;
    assert!(t1.contains("impl ByteNode {"));
    assert!(!t1.contains("impl ByteNode "));

    let t2 = word_list::WordMatch::load(FILE)?;
    assert!(t2.contains("impl ByteNode {"));
    assert!(!t2.contains("impl ByteNode "));

    let t3 = string_set::SetMatch::load(FILE)?;
    assert!(t3.contains("impl ByteNode {"));
    assert!(!t3.contains("impl ByteNode "));

    Ok(())
}
