use merk::*;
use rand::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::borrow::BorrowMut as _;
use std::fs;
use std::time::Instant;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // delete /tmp/merk.db file if it exists
    let _ = fs::remove_file("/tmp/merk.db");
    let mut merk = Merk::open("/tmp/merk.db").unwrap();

    // Prepare to measure insertion time

    let txs = 1000;
    let keys = 50;
    let keylen = 32; // Length of each key
    let valuelen = 256; // Length of each value

    let mut all_keys = Vec::new();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let start_time = Instant::now();

    // Commit 10,000 transactions each with 10 key-value pairs
    for _ in 0..txs {
        let mut transaction: Vec<(Vec<u8>, Op)> = Vec::new();
        for _ in 0..keys {
            let key = rng
                .borrow_mut()
                .sample_iter(&Alphanumeric)
                .take(keylen)
                .collect::<Vec<u8>>();
            let value = rng
                .borrow_mut()
                .sample_iter(&Alphanumeric)
                .take(valuelen)
                .collect::<Vec<u8>>();
            transaction.push((key.clone(), Op::Put(value)));
            all_keys.push(key);
        }
        transaction.sort_by(|a, b| a.0.cmp(&b.0));
        let batch: &[(Vec<u8>, Op)] = &transaction;
        merk.apply(batch, &[]).unwrap();
    }

    let insertion_duration = start_time.elapsed();
    println!(
        "Completed inserting {:?} key-value pairs. Time taken: {:?}",
        txs * keys,
        insertion_duration
    );

    // Prepare to measure reading time
    let start_time = Instant::now();

    // Verify all keys
    for key in all_keys {
        let _retrieved_value = merk.get(&key)?.ok_or("Value not found for key")?;
    }

    let reading_duration = start_time.elapsed();
    println!(
        "Completed reading all keys. Time taken: {:?}",
        reading_duration
    );

    Ok(())
}
