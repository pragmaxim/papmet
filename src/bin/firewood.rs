use firewood::{
    db::{BatchOp, Db, DbConfig},
    v2::api::{Db as _, Proposal},
};
use rand::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::sync::Arc;
use std::{borrow::BorrowMut as _, time::Instant};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // delete /tmp/firewood directory if it exists
    let _ = std::fs::remove_dir_all("/tmp/firewood");
    let cfg = DbConfig::builder().truncate(true).build();
    let db = Db::new("/tmp/firewood", &cfg)
        .await
        .expect("db initiation should succeed");

    let txs = 1000;
    let keys = 50;
    let keylen = 32; // Length of each key
    let valuelen = 256; // Length of each value

    let mut rng = rand::rngs::StdRng::from_entropy();
    let mut all_keys = Vec::new();

    let start_time = Instant::now();

    for _ in 0..txs {
        let mut transaction = Vec::new();
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
        txs * keys,
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
