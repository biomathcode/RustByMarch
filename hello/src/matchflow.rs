pub fn run(n: u8) {
    match n {
        0 => println!("this is the first number"),
        1 | 2 => println!("this is the second number"),
        3..=4 => println!("this is the fourth number"),
        _ => println!("sdlfkms"), // kinda like default in switch statement
    }
}
