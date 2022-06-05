pub fn run() {
    let str: &str = "hello world";
    let string: String = String::from("Hello world");

    let slice = &string[..6];
    println!("{} {}", slice, str);
}
