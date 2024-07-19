extern crate chrono;
use chrono::Utc;

pub fn handle() {
    // 获取当前时间
    let now = Utc::now();

    // 获取当前时间戳（秒）
    let timestamp = now.timestamp();

    // 获取当前时间戳（毫秒）
    let timestamp_millis = now.timestamp_millis();

    println!("当前时间戳（秒）：{}", timestamp);
    println!("当前时间戳（毫秒）：{}", timestamp_millis);
}