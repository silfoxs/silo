use uuid::Uuid;

pub fn handle(num: u32) {
    println!("生成的 UUIDv4:");
    for _ in 0..num {
        let my_uuid = Uuid::new_v4();
        println!("{}", my_uuid);
    }
}
