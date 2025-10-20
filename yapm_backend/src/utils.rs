use std::time::{SystemTime, UNIX_EPOCH};

pub fn getUnixTime() -> u64 {
    let now = SystemTime::now();

    let unix_time = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs(); // seconds since epoch

    unix_time
}
