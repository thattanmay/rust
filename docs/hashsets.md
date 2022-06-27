```rust
use std::collections::HashSet;

// create a HashSet
let mut hs: HashSet<i32> = HashSet::new();

// insert
hs.insert(1);
hs.insert(2);
hs.insert(3);

// iterate
for i in &hs {
    println!("{}", i);
}
```