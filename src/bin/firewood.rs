use firewood::{
    db::{BatchOp, Db, DbConfig},
    v2::api::{Db as _, Proposal},
};
use papmet::random::generate_kv;
use papmet::settings::*;
use rand::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // delete /tmp/firewood directory if it exists
    let _ = std::fs::remove_dir_all("/tmp/firewood");
    let cfg = DbConfig::builder().truncate(true).build();
    let db = Db::new("/tmp/firewood", &cfg)
        .await
        .expect("db initiation should succeed");

    let mut rng = rand::rngs::StdRng::from_entropy();
    let mut all_keys = Vec::new();

    let start_time = Instant::now();

    for _ in 0..TXS_COUNT {
        let mut transaction = Vec::new();
        for _ in 0..KEYS_COUNT {
            let (key, value) = generate_kv(&mut rng, KEY_LENGTH, VALUE_LENGTH);
            transaction.push(BatchOp::Put {
                key: key.clone(),
                value,
            });
            all_keys.push(key);
        }
        // Propose the batch and commit it
        let proposal = Arc::new(db.propose(transaction).await.unwrap());
        proposal.commit().await?;
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
    let rootHash = db.root_hash().await?;
    let revision = db.revision(rootHash).await?;
    for key in all_keys {
        revision.kv_get(key);
    }

    let reading_duration = start_time.elapsed();
    println!(
        "Completed reading all keys. Time taken: {:?}",
        reading_duration
    );

    Ok(())
}
