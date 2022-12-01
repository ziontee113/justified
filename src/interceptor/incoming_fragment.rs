use std::{fmt::Display, time::SystemTime};

#[derive(Debug, Clone)]
pub struct KeyIdentifier {
    device_alias: String,
    code: u16,
}

impl KeyIdentifier {
    pub fn device_alias(&self) -> &str {
        self.device_alias.as_ref()
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}

impl PartialEq for KeyIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.device_alias == other.device_alias
    }
}

#[derive(Debug, Clone)]
pub struct IncomingFragment {
    key: KeyIdentifier,
    value: i32,
    timestamp: SystemTime,
}

impl IncomingFragment {
    pub fn new(device_alias: &str, code: u16, value: i32, timestamp: SystemTime) -> Self {
        Self {
            key: KeyIdentifier {
                device_alias: device_alias.to_string(),
                code,
            },
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
        write!(f, "{}|{}", self.key.device_alias, self.key.code)
    }
}

#[cfg(test)]
mod fragment_test {
    use super::*;
    use crate::utils::mipoch;

    #[test]
    fn can_make_new_fragment() {
        let fragment = IncomingFragment::new("L1", 32, 1, mipoch(0));
        assert_eq!(fragment.key.device_alias, "L1");
        assert_eq!(fragment.key.code, 32);
        assert_eq!(fragment.value(), 1);
        assert_eq!(fragment.timestamp(), mipoch(0));
    }

    #[test]
    fn test_display_trait_implementation() {
        let fragment = IncomingFragment::new("L1", 32, 1, mipoch(0));
        assert_eq!(fragment.to_string(), "L1|32");

        let fragment = IncomingFragment::new("R2", 44, 0, mipoch(12));
        assert_eq!(fragment.to_string(), "R2|44");
    }
}
