use std::str::FromStr;

use evdev::Key;

#[cfg(test)]
mod test;

pub fn code_from_key(key: &str) -> Option<u16> {
    let mut final_key = key.to_uppercase();
    if !key.contains("BTN_") {
        final_key = format!("KEY_{}", final_key);
    }

    if let Ok(result) = Key::from_str(&final_key) {
        return Some(result.code());
    }
    None
}
