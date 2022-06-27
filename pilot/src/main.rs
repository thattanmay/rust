use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error in reading input.");
    input.trim().to_string()
}

// fn get_str_vec(s: String) -> Vec<String> {
//     s.split_whitespace().map(|x| x.to_string()).collect()
// }

fn main_logic(n: &i32) -> String {
    let mut vec: Vec<i32> = Vec::new();
    let mut val = *n;

    let mut flag = true;
    for i in (0..*n).rev() {
        vec.push(val);

        if flag {
            val -= i;
            flag = false;
        } else {
            val += i;
            flag = true;
        }
    }
    vec.iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let t = get_input().parse::<i32>().expect("Error parsing into int.");

    for _ in 0..t {
        let n = get_input().parse::<i32>().expect("Error parsing into int.");
        println!("{}", main_logic(&n));
    }
}

// 4 - 2 3 1 4
// 5 - 3 2 4 1 5
// 6 - 3 4 2 5 1 6
