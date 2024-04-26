// src/main.rs
use papmet::Base16PatriciaTrie;

fn main() {
    let mut trie = Base16PatriciaTrie::new("/tmp/rocksdb");

    // Hexadecimal keys and values
    trie.insert(b"1a3b".to_vec(), b"Hello, World!".to_vec());
    trie.insert(b"1a3c".to_vec(), b"Good Bye, World!".to_vec());

    // Retrieve values
    if let Some(value) = trie.get(b"1a3b".to_vec()) {
        println!("Value for '1a3b': {:?}", String::from_utf8(value));
    }

    trie.print();
}
