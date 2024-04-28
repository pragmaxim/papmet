// src/main.rs
use papmet::Base16PatriciaTrie;

fn main() {
    let mut trie: Base16PatriciaTrie<&[u8]> = Base16PatriciaTrie::new("/tmp/rocksdb");

    // Hexadecimal keys and values
    trie.insert(b"1a3b", b"Hello, World!");
    trie.insert(b"1a3c", b"Good Bye, World!");

    // retrieve value as Option and print it out otherwise print "Key not found"
    match trie.get(b"1a3b") {
        Some(value) => println!("{:?}", value),
        None => println!("Key not found"),
    }

    trie.print();
}
