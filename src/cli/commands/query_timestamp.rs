extern crate chrono;
use chrono::{NaiveDateTime, Local, DateTime, TimeZone};

pub fn handle(date_time: &str) {
    if date_time == "" {
        get_local_time()
    } else {
        format_date_time(date_time)
    }
}

fn get_local_time() {
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

fn format_date_time(date_time: &str) {
    let naive_datetime = NaiveDateTime::parse_from_str(date_time, "%Y-%m-%d %H:%M:%S").expect("datetime error");
    let local_datetime: DateTime<Local> = Local.from_local_datetime(&naive_datetime)
        .unwrap();


    let datetime = format!("{}", local_datetime.format("%Y-%m-%d %H:%M:%S"));
    println!("输入时间：{}", datetime);
    println!("时间戳: {}", local_datetime.timestamp());
}