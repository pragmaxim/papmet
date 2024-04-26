use papmet::Base16PatriciaTrie;

fn main() {
    let mut trie = Base16PatriciaTrie::new();
    trie.insert("1a3b", 5);
    trie.insert("1a3c", 7);
    trie.insert("1a3d", 10);

    trie.print();
}
