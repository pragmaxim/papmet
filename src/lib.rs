// src/lib.rs
use rocksdb::{Options, DB};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt::Debug;

pub struct TrieNode<T: Debug> {
    children: HashMap<u8, TrieNode<T>>,
    value: Option<T>,
    hash: Vec<u8>,
}

impl<T: Debug + Clone + AsRef<[u8]>> TrieNode<T> {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            value: None,
            hash: vec![],
        }
    }

    pub fn compute_hash(&mut self) {
        let mut hasher = Sha256::new();
        if let Some(ref value) = self.value {
            hasher.update(value);
        }
        for (&key, child) in self.children.iter() {
            hasher.update(&[key]);
            hasher.update(&child.hash);
        }
        self.hash = hasher.finalize().to_vec();
    }
}

pub struct Base16PatriciaTrie<T: Debug> {
    root: TrieNode<T>,
    db: DB,
}

impl<T: Debug + Clone + AsRef<[u8]>> Base16PatriciaTrie<T> {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).expect("Failed to open DB");
        Base16PatriciaTrie {
            root: TrieNode::new(),
            db,
        }
    }

    pub fn insert(&mut self, key: &[u8], value: T) {
        let key_bytes = key.to_vec();
        let mut node = &mut self.root;
        for byte in key_bytes.iter() {
            let index = *byte;
            node = node.children.entry(index).or_insert_with(TrieNode::new);
        }
        node.value = Some(value.clone());
        node.compute_hash();
        self.db
            .put(&key_bytes, value)
            .expect("Failed to write to DB");
        self.root.compute_hash();
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key).unwrap()
    }

    fn hash_to_hex_string(&self, hash: &Vec<u8>) -> String {
        hash.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    pub fn print(&self) {
        println!(
            "Base16 Patricia Trie: {}",
            self.hash_to_hex_string(&self.root.hash)
        );
        for (&key, child) in &self.root.children {
            println!("{}", (key as char)); // Print the root character of each branch
            self.print_from(child, "", true); // Start the recursive print
        }
    }

    // Recursive helper method to print the trie
    fn print_from(&self, node: &TrieNode<T>, prefix: &str, is_last: bool) {
        let indent = if is_last { "    " } else { "│   " };
        let children = node.children.iter().collect::<Vec<_>>();
        for (i, (&key, child)) in children.iter().enumerate() {
            let connector = if i == children.len() - 1 {
                "└── "
            } else {
                "├── "
            };
            // Convert byte to char for display, ensuring it's valid ASCII
            if let Some(ch) = char::from_u32(key as u32) {
                let new_prefix = format!("{}{}{}", prefix, connector, ch);
                // Check if node has a value and convert bytes to string for display
                if let Some(ref value) = child.value {
                    // Convert byte slice to string; handle invalid UTF-8 gracefully
                    let value_str = String::from_utf8_lossy(value.as_ref());
                    println!("{} ({})", new_prefix, value_str);
                } else {
                    println!("{}", new_prefix);
                }
                self.print_from(
                    child,
                    &(prefix.to_string() + indent),
                    i == children.len() - 1,
                );
            }
        }
    }
}
