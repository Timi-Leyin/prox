use std::fmt::Display;
use chrono::Utc;
pub fn logger<T: Display>(msg: T) {
    let d = Utc::now();
    println!("[LOG] {} {}",d, msg);
}