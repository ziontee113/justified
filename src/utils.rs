use std::time::{Duration, SystemTime};

pub fn millis_from_epoch(milis: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_millis(milis)
}
