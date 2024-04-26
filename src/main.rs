use papmet::PatriciaTrie;

fn main() {
    let mut trie = PatriciaTrie::new();
    trie.insert("hello", 5);
    trie.insert("hell", 7);
    trie.insert("helium", 10);
    trie.insert("foo", 1);
    trie.print();
}
