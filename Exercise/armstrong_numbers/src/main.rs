// 9 is an Armstrong number, because 9 = 9^1 = 9
// 10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
// 153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
// 154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

fn main() {
    println!("{} ", is_armstron(10));
    println!("{} ", is_armstron(9));
    println!("{} ", is_armstron(153));

    println!("{} ", is_armstron(1741725));
}

pub fn is_armstron(num: u32) -> bool {
    let num_len = num.to_string().len() as u32;
    (num as u64)
        == num
            .to_string()
            .chars()
            .fold(0, |t, x| (x.to_digit(10).unwrap() as u64).pow(num_len) + t)
}

// fold works like reducer
// t is the result of the next
// x is the iterator value
