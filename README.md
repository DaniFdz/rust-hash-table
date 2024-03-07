# Rust Hash Table
Implementation of Hash Table in Rust for recreational purposes.

### Time comparision of inserting 1e7 elements
| Implementation | Time |
| --- | --- |
| Mine | 762.745432ms |
| Rust | 792.445387ms |

I used SDBM algorith for hashing: [Reference](http://www.cse.yorku.ca/~oz/hash.html)

And Open Addressing with Linear Probing to handle collisions: [Reference](https://carmencincotti.com/2022-10-10/linear-probing-open-addressing-hash-tables/)

Therefore the Time Complexity will be:
|  Operation 📐 | Average ❔ | Worst case ❗ |
| --- | --- | --- |
| Insertion | O(1) | O(n) |
| Deletion | O(1) | O(n) |
| Lookup | O(1) | O(n) |

## 🚀 Quick Start

### Dependencies
```bash
cargo run --release
```

