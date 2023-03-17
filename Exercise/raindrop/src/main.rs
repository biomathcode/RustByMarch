// The rules of raindrops are that if a given number:

// has 3 as a factor, add 'Pling' to the result.
// has 5 as a factor, add 'Plang' to the result.
// has 7 as a factor, add 'Plong' to the result.
// does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.

fn main() {
    println!("{}", raindrops(105))
}

pub fn raindrops(n: u32) -> String {
    let mut response = String::from("");

    let factors = factor(n);

    if factors.contains(&3) {
        response += "Pling"
    }
    if factors.contains(&5) {
        response += "Plang"
    }
    if factors.contains(&7) {
        response += "Plong"
    }

    if response.len() == 0 {
        response = n.to_string()
    }

    response
}
fn factor(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    if n % 3 == 0 {
        vec.push(3)
    } else {
        vec.push(n)
    }
    if n % 5 == 0 {
        vec.push(5)
    } else {
        vec.push(n)
    }
    if n % 7 == 0 {
        vec.push(7)
    } else {
        vec.push(n)
    }
    vec
}
