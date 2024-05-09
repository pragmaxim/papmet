use merk::*;
use papmet::random::generate_kv;
use papmet::settings::*;
use rand::prelude::*;
use std::fs;
use std::time::Instant; // Add missing import // Import the common module

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // delete /tmp/merk.db file if it exists
    let _ = fs::remove_file("/tmp/merk.db");
    let mut merk = Merk::open("/tmp/merk.db").unwrap();

    // Prepare to measure insertion time
    let mut all_keys = Vec::new();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let start_time = Instant::now();

    // Commit 10,000 transactions each with 10 key-value pairs
    for _ in 0..TXS_COUNT {
        let mut transaction: Vec<(Vec<u8>, Op)> = Vec::new();
        for _ in 0..KEYS_COUNT {
            let (key, value) = generate_kv(&mut rng, KEY_LENGTH, VALUE_LENGTH);
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
        TXS_COUNT * KEYS_COUNT,
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
