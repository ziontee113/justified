#[cfg(test)]
mod test;

#[macro_export]
macro_rules! ki {
    ($a:ident $b:expr) => {
        $crate::units::key_identifier::KeyIdentifier::new(stringify!($a), $b)
    };
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct KeyIdentifier {
    device_alias: String,
    code: u16,
}

impl KeyIdentifier {
    pub fn new(device_alias: &str, code: u16) -> Self {
        Self {
            device_alias: device_alias.to_string(),
            code,
        }
    }

    pub fn device_alias(&self) -> &str {
        self.device_alias.as_ref()
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn is(&self, device_alias: &str, code: u16) -> bool {
        self.device_alias == device_alias && self.code == code
    }
}

impl std::fmt::Display for KeyIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.device_alias, self.code)
    }
}
