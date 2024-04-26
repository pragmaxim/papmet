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
pub struct Base16PatriciaTrie<T: Debug> {
    root: TrieNode<T>,
}

impl<T: Debug> Base16PatriciaTrie<T> {
    pub fn new() -> Self {
        Base16PatriciaTrie {
            root: TrieNode::new(),
        }
    }

    // Inserts a hex-encoded string into the trie with an associated value.
    pub fn insert(&mut self, hex_str: &str, value: T) {
        let mut current_node = &mut self.root;
        // Ensure that the string is interpreted as hex digits
        for ch in hex_str.chars() {
            if ch.is_digit(16) {
                // Only proceed if the character is a valid hex digit
                current_node = current_node
                    .children
                    .entry(ch.to_ascii_uppercase())
                    .or_insert_with(TrieNode::new);
            } else {
                eprintln!("Invalid hex character: {}", ch);
                return;
            }
        }
        current_node.value = Some(value);
    }

    pub fn print(&self) {
        println!("Base16 Patricia Trie:");
        for (&ch, child) in &self.root.children {
            println!("{}", ch);
            self.print_from(child, "", true);
        }
    }

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
