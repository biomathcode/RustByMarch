fn main() {
    println!(" {}", production_rate_per_hour(6));
    println!(" {}", working_items_per_minute(6));
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production: f64 = 221.00 * speed as f64;

    match speed {
        1..=4 => production,
        5..=8 => production * 0.9,
        9..=10 => production * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
