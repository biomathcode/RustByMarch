mod array;
mod mutability;
mod tuple;
mod variables;

fn main() {
    // variables::run();
    // array::array();
    tuple::main();

    mutability::mutability();

    let a = 10;
    let b = 15;
    println!("hello, world, {}", a + b);

    println!("{}", is_even(8));

    println!("{}", is_odd(8));
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return value
}

pub fn is_odd(num: u16) -> bool {
    let digit: u16 = num % 2;
    digit != 0 // return value
}
