fn main() {
    println!("{}", is_leap_year(1604))
}
pub fn is_leap_year(year: u64) -> bool {
    let result = (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0);
    result
}
