extern crate chrono;
use chrono::{Local};

pub fn handle() {
    // 获取当前时间
    let now = Local::now();

    // 获取当前时间戳（秒）
    let timestamp = now.timestamp();

    // 获取当前时间戳（毫秒）
    let timestamp_millis = now.timestamp_millis();

    let datetime = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));

    println!("当前时间戳（秒）：{}", timestamp);
    println!("当前时间戳（毫秒）：{}", timestamp_millis);
    println!("当前时间：{}", datetime);
}