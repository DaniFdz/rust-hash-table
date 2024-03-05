#[derive(Debug)]
struct HashTable<Key, Value> {
    kvs: Vec<(Key, Value)>,
}

impl<Key: Default + Clone, Value: Default + Clone> HashTable<Key, Value> {
    #[allow(dead_code)]
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 64;
        Self {
            kvs: vec![(Key::default(), Value::default()); INITIAL_CAPACITY],
        }
    }

    #[allow(dead_code)]
    fn insert(key: Key, value: Value) {
        todo!()
    }

    #[allow(dead_code)]
    fn get(key: &Key) -> Option<&Value> {
        todo!()
    }

    #[allow(dead_code)]
    fn get_mut(key: &Key) -> Option<&Value> {
        todo!()
    }

    #[allow(dead_code)]
    fn remove(key: Key) -> Option<Value> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hash_table() {
        let hash_table: HashTable<i32, i32> = HashTable::new();
        assert_eq!(hash_table.kvs.len(), 64); // Initial capacity
    }
}
