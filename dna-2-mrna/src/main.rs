use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let char_vec: Vec<char> = input.chars().collect();
    let mut new_char = String::new();
    for c in char_vec {
        match c {
            'a' => new_char += "u",
            't' => new_char += "a",
            'g' => new_char += "c",
            'c' => new_char += "g",
            _ => new_char += &c.to_string(),
        }
    }
    println!("{}", new_char);
}
