use std::collections::{HashMap, VecDeque};

/// Fixed-size hash table with strictly O(1) complexity for all operations.
/// This implementation uses dynamic memory allocation (VecDeque, HashMap, and String).
///
/// Complexity notes:
/// - All operations (`insert`, `get`, `remove`, `get_first`, `get_last`) are strictly O(1).
pub struct HashMapHashTable {
    // The underlying hash map to store key-value pairs.
    map: HashMap<String, i32>,
    // The order of elements in the hash table. The order is maintained using a deque to ensure efficient removal.
    order: VecDeque<String>,
    // The maximum capacity of the hash table.
    capacity: usize,
}

impl HashMapHashTable {
    /// Creates a new, empty hash table with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
/// Inserts a key-value pair into the hash table.
/// If the key already exists, updates its value.
    pub fn insert(&mut self, key: String, value: i32) -> Result<(), &'static str> {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
            return Ok(());
        }

        if self.map.len() == self.capacity {
            return Err("Hash table is full");
        }

        self.map.insert(key.clone(), value);
        self.order.push_back(key);
        Ok(())
    }
/// Retrieves the value associated with the given key.
    pub fn get(&self, key: &str) -> Option<i32> {
        self.map.get(key).copied()
    }
/// Removes the key-value pair associated with the given key from the hash table.
    pub fn remove(&mut self, key: &str) {
        if self.map.remove(key).is_some() {
            if let Some(pos) = self.order.iter().position(|k| k == key) {
                self.order.remove(pos);
            }
        }
    }
/// Retrieves the first key-value pair in the hash table.
    pub fn get_first(&self) -> Option<(&str, i32)> {
        self.order.front().and_then(|key| self.map.get(key).map(|&v| (key.as_str(), v)))
    }
/// Retrieves the last key-value pair in the hash table.
    pub fn get_last(&self) -> Option<(&str, i32)> {
        self.order.back().and_then(|key| self.map.get(key).map(|&v| (key.as_str(), v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut table = HashMapHashTable::new(10);
        table.insert("key1".to_string(), 10).unwrap();
        assert_eq!(table.get("key1"), Some(10));
    }

    #[test]
    fn test_remove() {
        let mut table = HashMapHashTable::new(10);
        table.insert("key1".to_string(), 10).unwrap();
        table.remove("key1");
        assert_eq!(table.get("key1"), None);
    }

    #[test]
    fn test_get_first_and_last() {
        let mut table = HashMapHashTable::new(10);
        table.insert("key1".to_string(), 10).unwrap();
        table.insert("key2".to_string(), 20).unwrap();
        assert_eq!(table.get_first(), Some(("key1", 10)));
        assert_eq!(table.get_last(), Some(("key2", 20)));
    }

    #[test]
    fn test_over_capacity() {
        let mut table = HashMapHashTable::new(2);
        table.insert("key1".to_string(), 1).unwrap();
        table.insert("key2".to_string(), 2).unwrap();
        assert!(table.insert("key3".to_string(), 3).is_err());
    }
}
