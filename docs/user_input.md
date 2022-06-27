```rust
use std::io;

// read line from user
let mut input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Error in reading input.");
input.trim().to_string()


```