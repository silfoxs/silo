extern crate md5;
use md5::{Digest, Md5};

pub fn handle(str: &str) {
    // 创建一个 Md5 哈希对象
    let mut hasher = Md5::new();

    // 将输入字符串添加到哈希对象中
    hasher.update(str);

    // 计算哈希值
    let result = hasher.finalize();

    // 将哈希值转换为十六进制字符串
    let hex_result = format!("{:x}", result);
    println!("MD5值: {}", hex_result);

}