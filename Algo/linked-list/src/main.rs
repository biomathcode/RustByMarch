#[derive(debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>));


fn main() {
    println!("Hello, world!");
}
