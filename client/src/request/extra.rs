pub fn extra() -> String {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    return format!("{}:{}", current_time.as_micros(), rand::random::<usize>());
}
