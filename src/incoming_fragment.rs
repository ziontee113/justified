use std::{fmt::Display, time::SystemTime};

#[derive(Debug, Clone)]
pub struct IncomingFragment {
    device_alias: String,
    code: u16,
    value: i32,
    timestamp: SystemTime,
}

impl IncomingFragment {
    pub fn new(device_alias: &str, code: u16, value: i32, timestamp: SystemTime) -> Self {
        Self {
            device_alias: device_alias.to_string(),
            code,
            value,
            timestamp,
        }
    }
}

impl IncomingFragment {
    pub fn device_alias(&self) -> &str {
        self.device_alias.as_ref()
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

impl Display for IncomingFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.device_alias, self.code)
    }
}

#[cfg(test)]
mod fragment_test {
    use super::*;
    use crate::utils::millis_from_epoch;

    #[test]
    fn test_display_trait_implementation() {
        let fragment = IncomingFragment::new("L1", 32, 1, millis_from_epoch(0));
        assert_eq!(fragment.to_string(), "L1|32");
    }
}
