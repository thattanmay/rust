```rust

let s = "hello world";

// convert a String to Vec<String>
s.split_whitespace().map(|x| x.to_string()).collect()

// reverse a string
s.iter()
    .rev()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()

// join a str
s.join(" ")

```