#[cfg(test)]
mod test;

use std::{fmt::Display, time::SystemTime};

use crate::units::KeyIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum KeyState {
    Down,
    Up,
    Hold,
    Uninitiated,
}

impl From<i32> for KeyState {
    fn from(value: i32) -> Self {
        match value {
            0 => KeyState::Up,
            1 => KeyState::Down,
            2 => KeyState::Hold,
            -1 => KeyState::Uninitiated,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IncomingFragment {
    key: KeyIdentifier,
    value: KeyState,
    timestamp: SystemTime,
}

impl IncomingFragment {
    /// # Examples:
    /// ```
    /// let fragment = IncomingFragment::new("L1", 34, 1, mipoch(0));
    /// assert_eq!(fragment.key.device_alias(), "L1");
    /// assert_eq!(fragment.key.code(), 32);
    /// assert_eq!(fragment.value(), KeyState::Down);
    /// assert_eq!(fragment.timestamp(), mipoch(0));
    /// ```
    pub fn new(device_alias: &str, code: u16, value: i32, timestamp: SystemTime) -> Self {
        Self {
            key: KeyIdentifier::new(device_alias, code),
            value: value.into(),
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
    pub fn value(&self) -> KeyState {
        self.value.clone()
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

impl Display for IncomingFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}
