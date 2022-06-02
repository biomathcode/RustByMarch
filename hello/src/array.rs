pub fn array() {
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}", arr[0]);

    println!("{:?}", other_arr)
}
