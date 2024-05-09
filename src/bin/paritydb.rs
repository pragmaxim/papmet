use papmet::random::generate_kv;
use papmet::settings::*;
use parity_db::{Db, Options};
use rand::prelude::*;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = Path::new("/tmp/paritydb");
    if !db_path.exists() {
        fs::create_dir_all(db_path)?;
    }

    let mut options = Options::with_columns(db_path, 1);
    options.sync_wal = true;
    options.sync_data = true;

    let db = Db::open_or_create(&options)?;

    let mut all_keys = Vec::new();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let start_time = Instant::now();

    // Commit transactions with key-value pairs
    for _ in 0..TXS_COUNT {
        let mut transaction = Vec::new();
        for _ in 0..KEYS_COUNT {
            let (key, value) = generate_kv(&mut rng, KEY_LENGTH, VALUE_LENGTH);
            transaction.push((0, key.clone(), Some(value)));
            all_keys.push(key);
        }
        db.commit(transaction)?;
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
        let _retrieved_value = db.get(0, &key)?.ok_or("Value not found for key")?;
    }

    let reading_duration = start_time.elapsed();
    println!(
        "Completed reading all keys. Time taken: {:?}",
        reading_duration
    );

    Ok(())
}
