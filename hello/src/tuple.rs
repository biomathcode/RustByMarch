pub fn main() {
    let tuple: (u8, bool, f32) = (5, true, 2.1);

    let tuple2 = (3, 5);

    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);

    // desctructing
    let (a, b) = tuple2;

    println!("first {}, second {}", a, b);
}
