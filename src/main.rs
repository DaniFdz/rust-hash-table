use std::collections::HashMap;

use rust_hash_table::hash_table::HashTable;

const ITERATIONS: i32 = 1e7 as i32;

fn main() {
    let start = std::time::Instant::now();
    let mut hash_table: HashTable<i32, i32> = HashTable::new();
    for i in 0..ITERATIONS {
        hash_table.insert(i, 0);
    }
    println!("My implementation time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..ITERATIONS {
        hash_map.insert(i, 0);
    }
    println!("Rust implementation time: {:?}", start.elapsed());
}
