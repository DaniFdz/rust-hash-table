use rust_hash_table::hash_table::HashTable;

fn main() {
    let mut hash_table: HashTable<i32, i32> = HashTable::new();
    for i in 0..65{
        hash_table.insert(i as i32, 0);
    }
    println!("Len: {}, Size: {}", hash_table.len(), hash_table.size());
}
