Learning Rust by implementing Persistent Authenticated Merkle Trie datastructure for holding state of decentralized software like Blockchain.
```
cargo run --bin papmet
```

There are 3 benchmarks of real storages included : 
```
cargo run --bin firewood // Compaction-Less DB for Merkleized Blockchain State
cargo run --bin paritydb // Patricia-Merkle trie based storage backed by RocksDB
cargo run --bin merk     // Merkle AVL tree built on top of RocksDB
cargo run --bin grovedb  // Hierarchical Authenticated Data Structure on top of Merk and RocksDB
```

See [firewood](https://github.com/ava-labs/firewood), [paritydb](https://github.com/paritytech/parity-db), [merk](https://github.com/turbofish-org/merk) and [grovedb](https://github.com/dashpay/grovedb)