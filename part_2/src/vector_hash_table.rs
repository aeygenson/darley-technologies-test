use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Fixed-size hash table using linear probing for collision resolution.
/// This implementation uses dynamic memory allocation (Vec and String).
///
/// Complexity notes:
/// - `insert`: O(1) on average; O(n) worst-case when the table is nearly full or many collisions occur.
/// - `remove`: O(1) on average; O(n) worst-case due to potential collisions.
/// - `get`: O(1) on average; O(n) worst-case due to potential collisions.
pub struct VectorHashTable {
    // The underlying hash table to store key-value pairs.
    table: Vec<Option<(String, i32)>>,
    // The maximum capacity of the hash table.
    capacity: usize,
}

impl VectorHashTable {
    /// Creates a new, empty hash table with the given size.
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![None; size],
            capacity: size,
        }
    }
/// Generates a hash value for the given key.
    fn hash(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }
/// Inserts a key-value pair into the hash table.
/// if the key already exists, updates its value.
/// check if the table is full before inserting a new key.
    pub fn insert(&mut self, key: String, value: i32) -> Result<(), &'static str> {
        let mut idx = self.hash(&key);
        for _ in 0..self.capacity {
            match &self.table[idx] {
                None => {
                    self.table[idx] = Some((key, value));
                    return Ok(());
                }
                Some((existing_key, _)) if existing_key == &key => {
                    self.table[idx] = Some((key, value));
                    return Ok(());
                }
                _ => idx = (idx + 1) % self.capacity,
            }
        }
        Err("Hash table is full")
    }
/// Retrieves the value associated with the given key.
    pub fn get(&self, key: &str) -> Option<i32> {
        let mut idx = self.hash(key);
        for _ in 0..self.capacity {
            match &self.table[idx] {
                None => return None,
                Some((existing_key, value)) if existing_key == key => return Some(*value),
                _ => idx = (idx + 1) % self.capacity,
            }
        }
        None
    }
/// Removes the key-value pair associated with the given key from the hash table.
    pub fn remove(&mut self, key: &str) {
        let mut idx = self.hash(key);
        for _ in 0..self.capacity {
            match &self.table[idx] {
                None => return,
                Some((existing_key, _)) if existing_key == key => {
                    self.table[idx] = None;
                    return;
                }
                _ => idx = (idx + 1) % self.capacity,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut table = VectorHashTable::new(10);
        table.insert("key1".to_string(), 10).unwrap();
        assert_eq!(table.get("key1"), Some(10));
    }

    #[test]
    fn test_remove() {
        let mut table = VectorHashTable::new(10);
        table.insert("key1".to_string(), 10).unwrap();
        table.remove("key1");
        assert_eq!(table.get("key1"), None);
    }

    #[test]
    fn test_over_capacity() {
        let mut table = VectorHashTable::new(2);
        table.insert("key1".to_string(), 1).unwrap();
        table.insert("key2".to_string(), 2).unwrap();
        assert!(table.insert("key3".to_string(), 3).is_err());
    }
}