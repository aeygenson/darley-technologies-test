use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Constants for the fixed-size hash table.
/// CAPACITY to store key-value pairs.
pub const CAPACITY: usize = 12134;
/// Maximum length of a key.
pub const MAX_KEY_LEN: usize = 30;

/// Fixed-size hash table using linear probing for collision resolution.
/// This implementation uses only static memory allocation.
///
/// Complexity notes:
/// - `insert`: O(1) on average; O(n) worst-case when the table is nearly full or many collisions occur.
/// - `remove`: O(1) on average; O(n) worst-case if removing the first or last element triggers full scan to update pointers.
/// - `get`: O(1) on average; O(n) worst-case due to potential collisions.
/// - `get_first` and `get_last`: Generally O(1), but can degrade to O(n) if removals cause pointer updates through scanning.
pub struct ArrayHashTable {
    // Table is represented as an array of Option<([u8; MAX_KEY_LEN], usize, i32)>.
    // ([u8; MAX_KEY_LEN]: The actual key bytes, usize: Length of the key, i32: The associated value.)
    table: [Option<([u8; MAX_KEY_LEN], usize, i32)>; CAPACITY],
    // Pointers to the first and last elements in the table.
    first: Option<usize>,
    last: Option<usize>,
}

impl ArrayHashTable {
    /// Creates a new, empty fixed-size hash table.
    pub fn new() -> Self {
        Self {
            table: std::array::from_fn(|_| None),
            first: None,
            last: None,
        }
    }

    /// Generates a hash value for the given key.
    fn hash(&self, key: &[u8]) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % CAPACITY
    }
    /// Inserts a key-value pair into the hash table. 
    /// If the key already exists, updates its value.
    /// Also check if the table is full before inserting a new key.
    pub fn insert(&mut self, key: &[u8], value: i32) -> Result<(), &'static str> {
        if key.len() > MAX_KEY_LEN {
            return Err("Key length exceeds maximum allowed length");
        }

        let mut padded_key = [0u8; MAX_KEY_LEN];
        let key_len = key.len();
        padded_copy(key, &mut padded_key);

        let mut idx = self.hash(key);
        for _ in 0..CAPACITY {
            match &self.table[idx] {
                None => {
                    self.table[idx] = Some((padded_key, key_len, value));
                    if self.first.is_none() {
                        self.first = Some(idx);
                    }
                    self.last = Some(idx);
                    return Ok(());
                }
                Some((existing_key, existing_len, _)) if &existing_key[..*existing_len] == key => {
                    self.table[idx] = Some((padded_key, key_len, value));
                    self.last = Some(idx);
                    return Ok(());
                }
                _ => idx = (idx + 1) % CAPACITY,
            }
        }
        Err("Hash table is full")
    }
/// Removes a key-value pair from the hash table.
/// If removing the first or last element triggers a full scan to update pointers,
    pub fn remove(&mut self, key: &[u8]) {
        let mut idx = self.hash(key);
        loop {
            match &self.table[idx] {
                None => return,
                Some((existing_key, existing_len, _)) if &existing_key[..*existing_len] == key => {
                    self.table[idx] = None;
                    if self.first == Some(idx) || self.last == Some(idx) {
                        self.update_first_last();
                    }
                    return;
                }
                _ => idx = (idx + 1) % CAPACITY,
            }
        }
    }
/// Retrieves the value associated with the given key.
/// If the key does not exist, returns None.
    pub fn get(&self, key: &[u8]) -> Option<i32> {
        let mut idx = self.hash(key);
        loop {
            match &self.table[idx] {
                None => return None,
                Some((existing_key, existing_len, value)) if &existing_key[..*existing_len] == key => return Some(*value),
                _ => idx = (idx + 1) % CAPACITY,
            }
        }
    }
/// Returns the last key-value pair in the table.
    pub fn get_last(&self) -> Option<(&[u8], i32)> {
        self.last.and_then(|idx| self.table[idx].as_ref().map(|(k, len, v)| (&k[..*len], *v)))
    }
/// Returns the first key-value pair in the table.
    pub fn get_first(&self) -> Option<(&[u8], i32)> {
        self.first.and_then(|idx| self.table[idx].as_ref().map(|(k, len, v)| (&k[..*len], *v)))
    }
/// Updates the first and last pointers if necessary after a removal.
    fn update_first_last(&mut self) {
        self.first = None;
        self.last = None;
        for (idx, entry) in self.table.iter().enumerate() {
            if entry.is_some() {
                if self.first.is_none() {
                    self.first = Some(idx);
                }
                self.last = Some(idx);
            }
        }
    }
}
/// Helper function to copy a slice into a fixed-size array.
fn padded_copy(src: &[u8], dst: &mut [u8; MAX_KEY_LEN]) {
    dst[..src.len()].copy_from_slice(src);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut table = ArrayHashTable::new();
        table.insert(b"key1", 10).unwrap();
        table.insert(b"key2", 20).unwrap();
        assert_eq!(table.get(b"key1"), Some(10));
        assert_eq!(table.get(b"key2"), Some(20));
    }

    #[test]
    fn test_remove() {
        let mut table = ArrayHashTable::new();
        table.insert(b"key1", 10).unwrap();
        table.remove(b"key1");
        assert!(table.get(b"key1").is_none());
    }

    #[test]
    fn test_get_first_and_last() {
        let mut table = ArrayHashTable::new();
        table.insert(b"key1", 10).unwrap();
        table.insert(b"key2", 20).unwrap();
        assert_eq!(table.get_first(), Some((&b"key1"[..], 10)));
        assert_eq!(table.get_last(), Some((&b"key2"[..], 20)));
    }

    #[test]
    fn test_update_existing_key() {
        let mut table = ArrayHashTable::new();
        table.insert(b"key1", 10).unwrap();
        table.insert(b"key1", 30).unwrap();
        assert_eq!(table.get(b"key1"), Some(30));
        assert_eq!(table.get_first(), Some((&b"key1"[..], 30)));
        assert_eq!(table.get_last(), Some((&b"key1"[..], 30)));
    }

    #[test]
    fn test_insert_over_capacity() {
        let mut table = ArrayHashTable::new();
        for i in 0..CAPACITY {
            assert!(table.insert(format!("key{}", i).as_bytes(), i as i32).is_ok());
        }
        assert!(table.insert(b"overflow_key", 9999).is_err());
    }

    #[test]
    fn test_max_key_length() {
        let mut table = ArrayHashTable::new();
        let long_key = [b'a'; MAX_KEY_LEN + 1];
        assert!(table.insert(&long_key, 100).is_err());
    }
}
