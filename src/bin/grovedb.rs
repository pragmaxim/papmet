use grovedb::{Element, GroveDb, PathQuery, Query};
use papmet::random::generate_kv;
use papmet::settings::*;
use rand::prelude::*;
use std::fs;
use std::time::Instant; // Add missing import // Import the common module

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // delete /tmp/grove.db file if it exists
    let _ = fs::remove_file("/tmp/grove.db");

    // Specify a path and open GroveDB at the path as db
    let path = String::from("/tmp/grove.db");
    let db = GroveDb::open(path).unwrap();

    // Prepare to measure insertion time
    let mut all_keys = Vec::new();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let start_time = Instant::now();

    let root_path: &[&[u8]] = &[];

    // Commit transactions with key-value pairs
    for _ in 0..TXS_COUNT {
        let mut transaction: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        for _ in 0..KEYS_COUNT {
            let (key, value) = generate_kv(&mut rng, KEY_LENGTH, VALUE_LENGTH);
            transaction.push((key.clone(), value.to_vec()));
            all_keys.push((key, value));
        }
        transaction.sort_by(|a, b| a.0.cmp(&b.0));

        let tx = db.start_transaction();
        for (k, v) in transaction {
            db.insert(root_path, &k, Element::new_item(v), None, Some(&tx))
                .unwrap()
                .unwrap();
        }
        db.commit_transaction(tx).unwrap().unwrap();
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
    for (key, value) in all_keys.iter() {
        let mut query = Query::new();
        query.insert_key(key.clone());
        let path_query = PathQuery::new_unsized(vec![], query.clone());
        let (elements, _) = db
            .query_item_value(&path_query, true, None)
            .unwrap()
            .expect("expected successful get_path_query");

        assert_eq!(elements.len(), 1);

        let proof = db.prove_query(&path_query).unwrap().unwrap();

        let (hash, result_set) = GroveDb::verify_query(&proof, &path_query).unwrap();

        assert_eq!(db.root_hash(None).unwrap().unwrap(), hash);

        assert_eq!(result_set.len(), 1);

        assert_eq!(
            result_set[0],
            (
                vec![].to_vec(),
                key.clone(),
                Some(Element::new_item(value.clone()))
            )
        );
    }

    let reading_duration = start_time.elapsed();
    println!(
        "Completed reading all keys. Time taken: {:?}",
        reading_duration
    );

    Ok(())
}
