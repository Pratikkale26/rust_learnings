use chrono::{Utc, Local};

fn main() {
    println!("Hello, world!");
    let utc = Utc::now();
    let local_time = Local::now();

    println!("utc time now - {}", utc);
    println!("my local time now - {}", local_time);
}
