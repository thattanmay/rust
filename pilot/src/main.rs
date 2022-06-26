use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error in reading input.");
    input.trim().to_string()
}

fn get_str_vec(s: String) -> Vec<String> {
    s.split_whitespace().map(|x| x.to_string()).collect()
}

fn main() {
    let t = get_str_vec(get_input())[0]
        .parse::<i32>()
        .expect("Error parsing into int.");

    for _ in 0..t {
        let input = get_str_vec(get_input());
        let a = input[0].parse::<i32>().expect("Error parsing into int.");
        let b = input[1].parse::<i32>().expect("Error parsing into int.");
        println!("{}", a);
        println!("{}", b);
    }
}
