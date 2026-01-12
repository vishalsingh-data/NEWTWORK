use chrono::Local;

/// Logs an informational message with a timestamp
pub fn log_info(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("[INFO][{}] {}", timestamp, msg);
}

/// Logs an error message with a timestamp
pub fn log_error(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    eprintln!("[ERROR][{}] {}", timestamp, msg);
}
