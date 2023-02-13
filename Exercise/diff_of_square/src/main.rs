fn main() {
    println!("{}", difference(10))
}

pub fn square_of_sum(n: u32) -> u32 {
    let mut result = 0;
    for i in 0..n {
        result += i + 1
    }

    result * result
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut result = 0;
    for j in 0..n {
        result += (j + 1) * (j + 1)
    }
    result
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
