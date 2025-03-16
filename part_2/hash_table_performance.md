# 🚀 Performance Analysis of Hash Table Implementations

## 📊 Benchmark Results

| Hash Table        | Insert Time | Get Time |
|------------------|------------|----------|
| **ArrayHashTable** | **71.20 ms** | **6.53 ms** |
| **VectorHashTable** | 118.84 ms | 18.40 ms |
| **HashMapHashTable** | 102.55 ms | **4.57 ms** |

---

## 🔍 Key Observations

### 🏆 **ArrayHashTable is still the fastest for insertion**
- **71.20ms vs. 118.84ms (Vector) vs. 102.55ms (HashMap)**
- Uses a **fixed-size array**, avoiding dynamic memory allocation.
- **Limitation:** Cannot dynamically grow beyond its allocated capacity.

### 📉 **VectorHashTable remains the slowest in both insert and get operations**
- **Insertion (118.84ms)** is the slowest due to:
    - **Linear probing** for collision handling.
    - Memory overhead from `Vec<Option<(String, i32)>>`.
- **Get time (18.40ms)** is also higher, likely due to the same linear scan overhead.

### 🚀 **HashMapHashTable is the fastest for retrieval**
- **4.57ms vs. 6.53ms (Array) vs. 18.40ms (Vector)**
- **`std::collections::HashMap`** provides near O(1) lookups.
- **Insertion time (102.55ms)** is slower due to:
    - **Dynamic memory allocation**
    - **Rehashing costs** when resizing.

---

## 📝 Final Conclusion

| Use Case | Best Hash Table |
|----------|----------------|
| **Fast Insertions & Known Capacity** | 🏆 `ArrayHashTable` |
| **Fast Lookups & Dynamic Growth** | 🚀 `HashMapHashTable` |
| **Hybrid, but not optimal** | ⚖️ `VectorHashTable` |

### **🛠 Recommendations Based on Use Case**
- **Use `ArrayHashTable`** when working with a **fixed number of elements** and prioritizing fast insertions.
- **Use `HashMapHashTable`** when handling **dynamic-sized data** where lookups are frequent.
- **Avoid `VectorHashTable`** unless necessary, as it provides **no clear advantage** over the other two.

---

🚀 **Final Takeaway:** **Use `ArrayHashTable` for fixed-size fast inserts, and `HashMapHashTable` for dynamic fast lookups.**
