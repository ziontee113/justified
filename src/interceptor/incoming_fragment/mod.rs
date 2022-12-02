#[cfg(test)]
mod test;

use std::{fmt::Display, time::SystemTime};

use crate::units::KeyIdentifier;

#[derive(Debug, Clone)]
pub struct IncomingFragment {
    key: KeyIdentifier,
    value: i32,
    timestamp: SystemTime,
}

impl IncomingFragment {
    pub fn new(device_alias: &str, code: u16, value: i32, timestamp: SystemTime) -> Self {
        Self {
            key: KeyIdentifier::new(device_alias.to_string(), code),
            value,
            timestamp,
        }
    }

    pub fn key(&self) -> &KeyIdentifier {
        &self.key
    }

    pub fn has_same_key(&self, other: &IncomingFragment) -> bool {
        self.key() == other.key()
    }
}

impl IncomingFragment {
    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

impl Display for IncomingFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.key.device_alias(), self.key.code())
    }
}
