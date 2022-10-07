```rust

let s = "hello world";

// convert a String to Vec<String>
s.split_whitespace().map(|x| x.to_string()).collect();
// convert a String to Vec<i32>
s.split_whitespace().map(|x| x.parse::<i32>().expect("Error parsing into int.")).collect();

// iterator over the chars of a string slice
s.chars()
    .map(|x| match x {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => 'X',
    })
    .collect::<String>()

```