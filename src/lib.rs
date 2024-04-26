// src/lib.rs

use std::collections::HashMap;
use std::fmt::Debug; // Import Debug to ensure that T can be formatted

#[derive(Debug)]
pub struct TrieNode<T: Debug> {
    children: HashMap<char, TrieNode<T>>,
    value: Option<T>,
}

impl<T: Debug> TrieNode<T> {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            value: None,
        }
    }
}

#[derive(Debug)]
pub struct PatriciaTrie<T: Debug> {
    root: TrieNode<T>,
}

impl<T: Debug> PatriciaTrie<T> {
    pub fn new() -> Self {
        PatriciaTrie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str, value: T) {
        let mut current_node = &mut self.root;
        for ch in word.chars() {
            current_node = current_node
                .children
                .entry(ch)
                .or_insert_with(TrieNode::new);
        }
        current_node.value = Some(value);
    }
}
