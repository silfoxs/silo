use uuid::Uuid;

pub fn handle(uuid: &str) {
    match Uuid::parse_str(uuid) {
        Ok(parsed_uuid) => println!("有效的 UUID: {}", parsed_uuid),
        Err(_) => eprintln!("无效的 UUID: {}", uuid),
    }
}
