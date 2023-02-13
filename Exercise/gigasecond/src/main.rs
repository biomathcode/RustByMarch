use time::Duration;
use time::OffsetDateTime;
use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    let n = i32::pow(10, 9);
    start + Duration::seconds(n.into())
}

fn main() {
    let now = OffsetDateTime::now_utc();
    let start = DateTime::new(now.date(), now.time());

    println!("{:?}", now);
    println!("{:?}", start);
    println!("{:?}", after(start));
}
