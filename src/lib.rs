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

    pub fn print(&self) {
        println!("Patricia Trie:");
        for (&ch, child) in &self.root.children {
            println!("{}", ch);
            self.print_from(child, "", true);
        }
    }

    // Recursive helper method to print the trie
    fn print_from(&self, node: &TrieNode<T>, prefix: &str, is_last: bool) {
        let indent = if is_last { "    " } else { "│   " };
        let children = node.children.iter().collect::<Vec<_>>();
        for (i, (&ch, child)) in children.iter().enumerate() {
            let connector = if i == children.len() - 1 {
                "└── "
            } else {
                "├── "
            };
            let new_prefix = format!("{}{}{}", prefix, connector, ch);
            if let Some(ref value) = child.value {
                println!("{} ({:?})", new_prefix, value);
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
