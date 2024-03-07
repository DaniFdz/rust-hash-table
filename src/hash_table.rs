use bincode::serialize;
use serde::ser::Serialize;
use std::cmp::PartialEq;

const INITIAL_CAPACITY: usize = 64;

// The hash function takes a reference to a key and returns a usize
//
// To calculate the hash we use the following steps:
// 1. Convert the key to a byte array
// 2. Iterate over the bytes and calculate the hash with sdbm algorithm
//      - http://www.cse.yorku.ca/~oz/hash.html
// 3. Return the hash
fn hash<K: Serialize>(key: &K) -> usize {
    let bytes = serialize(key).unwrap();
    let mut hash = 0;
    for byte in bytes {
        hash = (byte as usize)
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }
    hash
}

// The HashElement struct needs to types parameters, Key and Value
// The Key and Value types must implement the Default and Clone traits
#[derive(Debug, Clone)]
struct HashElement<Key, Value> {
    key: Key,
    value: Value,
    default: bool,
    deleted: bool,
}

// The HashTable struct has two type parameters, Key and Value
// The Key and Value types must implement the Default and Clone traits
// The HashTable has a kvs field which is a Vec of tuples of Key and Value types
// The HashTable has a len field which is the number of elements in the hash table
// The HashTable has a size field which is the number of elements the hash table can hold without resizing
#[derive(Debug)]
pub struct HashTable<Key, Value> {
    kvs: Vec<HashElement<Key, Value>>,
    len: usize,
}

// Implementation of the HashTable
//
// The Key and Value types must implement the Default and Clone traits
// The HashTable has a kvs field which is a Vec of tuples of Key and Value types
//
// When a collision occurs we use Open Addressing with Linear Probing to handle it
impl<Key: Default + Clone + Serialize + PartialEq, Value: Default + Clone> HashTable<Key, Value> {
    // Returns a new HashTable with an initial capacity of <INITIAL_CAPACITY>
    pub fn new() -> Self {
        Self {
            kvs: vec![
                HashElement {
                    key: Key::default(),
                    value: Value::default(),
                    default: true,
                    deleted: false,
                };
                INITIAL_CAPACITY
            ],
            len: 0,
        }
    }

    // Returns a new HashTable from a Vec of tuples
    pub fn from(v: Vec<(Key, Value)>) -> Self {
        let mut hash_table = HashTable::new();
        for (key, value) in v {
            hash_table.insert(key, value);
        }
        hash_table
    }

    // Returns if the Hash Table is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Returns the number of elements in the hash table
    pub fn len(&self) -> usize {
        self.len
    }

    // Returns how many elements the hash table can hold without resizing
    pub fn size(&self) -> usize {
        self.kvs.len()
    }

    // Calculates the load factor of the hash table with the formula:
    // load factor = number of elements / size of the hash table
    // The load factor is a measure of how full the hash table is
    fn load_factor(&self) -> f64 {
        self.len as f64 / self.size() as f64
    }

    // The resize method doubles the size of the hash table when needed
    fn resize(&mut self) {
        self.kvs.resize(
            self.size() * 2,
            HashElement {
                key: Key::default(),
                value: Value::default(),
                default: true,
                deleted: false,
            },
        );
    }

    // The insert method takes a key and a value and inserts the key-value pair into the hash table
    //
    // If the key already exists in the hash table, the value is updated
    // If the key does not exist in the hash table, the key-value pair is inserted
    //
    // If the hash table is at least at 70% capacity, the hash table is resized
    //
    // The key-value pair is inserted using Open Addressing with Linear Probing
    // If a collision occurs, the next available slot is used
    // If the hash table is full, the hash table is resized
    pub fn insert(&mut self, key: Key, value: Value) {
        // Check if we have to resize the vector of positions
        if self.load_factor() > 0.7 {
            self.resize()
        }

        // Calculate the position
        let hash = hash(&key);
        let mut pos = hash % self.size();

        // Find the next available position
        while !self.kvs[pos].default && !self.kvs[pos].deleted {
            if self.kvs[pos].key == key {
                self.kvs[pos].value = value;
                return;
            }
            pos = (pos + 1) % self.size();
        }

        // Insert the key-value pair
        self.kvs[pos] = HashElement {
            key,
            value,
            default: false,
            deleted: false,
        };
        self.len += 1;
    }

    // The get method takes a key and returns its value in the hash table
    //
    // If the value exist it will return Some(value)
    // Otherwise it will return None
    //
    // First it will calculate the hash of the key to get the position
    //
    pub fn get(&self, key: &Key) -> Option<&Value> {
        // Calculate the position
        let hash = hash(&key);
        let mut pos = hash % self.size();

        // Find the element in the hash table
        while !self.kvs[pos].default && !self.kvs[pos].deleted {
            if self.kvs[pos].key == *key {
                return Some(&self.kvs[pos].value);
            }
            pos = (pos + 1) % self.size();
        }

        // The element does not exist
        None
    }

    // The get_mut method takes a key and returns a mutable reference to its value in the hash table
    //
    // If the value exist it will return Some(&mut value)
    // Otherwise it will return None
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        // Calculate the position
        let hash = hash(&key);
        let mut pos = hash % self.size();

        // Find the element in the hash table
        while !self.kvs[pos].default && !self.kvs[pos].deleted {
            if self.kvs[pos].key == *key {
                return Some(&mut self.kvs[pos].value);
            }
            pos = (pos + 1) % self.size();
        }

        // The element does not exist
        None
    }

    // The remove method takes a key and removes its value from the hash table
    //
    // Instead of removing the element, we mark it as deleted
    // This is because we are using Open Addressing with Linear Probing
    // If we remove the element, we will not be able to find the next element in the sequence
    //
    // If the value exist it will return Some(value)
    // Otherwise it will return None
    pub fn remove(&mut self, key: Key) -> Option<Value> {
        // Calculate the position
        let hash = hash(&key);
        let mut pos = hash % self.size();

        // Find the element in the hash table
        while pos < self.size() && !self.kvs[pos].default && !self.kvs[pos].deleted {
            if self.kvs[pos].key == key {
                self.kvs[pos].deleted = true;
                self.len -= 1;
                return Some(self.kvs[pos].value.clone());
            }
            pos = (pos + 1) % self.size();
        }

        // The element does not exist
        None
    }
}

impl<Key: Default + Clone + Serialize + PartialEq, Value: Default + Clone> Default
    for HashTable<Key, Value>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hash_table() {
        let hash_table: HashTable<i32, i32> = HashTable::new();
        assert_eq!(hash_table.kvs.len(), INITIAL_CAPACITY);
    }

    #[test]
    fn test_len_when_empty() {
        let hash_table: HashTable<i32, i32> = HashTable::new();
        assert_eq!(hash_table.len(), 0);
    }

    #[test]
    fn test_is_empty_when_empty() {
        let hash_table: HashTable<i32, i32> = HashTable::new();
        assert!(hash_table.is_empty());
    }

    #[test]
    fn test_is_empty_when_not_empty() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        hash_table.insert(0, 0);
        assert!(!hash_table.is_empty());
    }

    #[test]
    fn test_size_when_initialized() {
        let hash_table: HashTable<i32, i32> = HashTable::new();
        assert_eq!(hash_table.size(), INITIAL_CAPACITY);
    }

    #[test]
    fn test_insert_increments_length() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        hash_table.insert(0, 0);
        assert_eq!(hash_table.len(), 1);
    }

    #[test]
    fn test_insert_string_does_not_panic() {
        let mut hash_table: HashTable<String, String> = HashTable::new();
        hash_table.insert("hello".to_string(), "world".to_string());
    }

    #[test]
    fn test_resizing_when_collision() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        for i in 0..(INITIAL_CAPACITY + 1) {
            hash_table.insert(i.try_into().unwrap(), 0);
        }
        assert_eq!(hash_table.len(), INITIAL_CAPACITY + 1);
        assert!(hash_table.size() > 1);
    }

    #[test]
    fn test_get_load_factor() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        for i in 0..(INITIAL_CAPACITY / 2) as usize {
            hash_table.insert(i.try_into().unwrap(), 0);
        }
        assert_eq!(hash_table.load_factor(), 0.5);
    }

    #[test]
    fn test_resize_when_load_factor_bigger_than_70_percent() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        for i in 0..(INITIAL_CAPACITY * 8 / 10) as usize {
            hash_table.insert(i.try_into().unwrap(), 0);
        }
        assert!(hash_table.load_factor() < 8.0 / 20.0);
    }

    #[test]
    fn test_insert_and_get() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        hash_table.insert(0, 0);
        assert_eq!(hash_table.get(&0), Some(&0));
    }

    #[test]
    fn test_get_when_key_does_not_exist() {
        let hash_table: HashTable<i32, i32> = HashTable::new();
        assert_eq!(hash_table.get(&0), None);
        assert_eq!(hash_table.get(&1), None);
    }

    #[test]
    fn test_cannot_update_with_get() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        hash_table.insert(0, 0);
        let mut _value = *hash_table.get(&0).unwrap();
        _value = 1;
        assert_eq!(hash_table.get(&0), Some(&0));
        assert_eq!(_value, 1);
    }

    #[test]
    fn test_get_mutable_reference() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        hash_table.insert(0, 0);
        let value = hash_table.get_mut(&0).unwrap();
        *value = 1;
        assert_eq!(hash_table.get(&0), Some(&1));
    }

    #[test]
    fn test_remove() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        hash_table.insert(0, 0);
        assert_eq!(hash_table.remove(0), Some(0));
        assert_eq!(hash_table.get(&0), None);
    }

    #[test]
    fn test_remove_when_key_does_not_exist() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        assert_eq!(hash_table.remove(0), None);
    }

    #[test]
    fn test_insert_after_removed() {
        let mut hash_table: HashTable<i32, i32> = HashTable::new();
        hash_table.insert(0, 0);
        hash_table.remove(0);
        assert_eq!(hash_table.len(), 0);
        hash_table.insert(0, 1);
        assert_eq!(hash_table.get(&0), Some(&1));
        assert_eq!(hash_table.len(), 1);
    }

    #[test]
    fn test_hash_table_from_vector() {
        let hash_table = HashTable::from(vec![(0, 0), (1, 1), (2, 2)]);
        assert_eq!(hash_table.get(&0), Some(&0));
        assert_eq!(hash_table.get(&1), Some(&1));
        assert_eq!(hash_table.get(&2), Some(&2));
        assert_eq!(hash_table.get(&3), None);
        assert_eq!(hash_table.len(), 3);
    }
}
