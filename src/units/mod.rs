#[cfg(test)]
mod test;

#[macro_export]
macro_rules! ki {
    ($a:ident $b:expr) => {
        $crate::units::KeyIdentifier::new(stringify!($a), $b)
    };
}

#[derive(Debug, Clone)]
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
}

impl std::fmt::Display for KeyIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.device_alias, self.code)
    }
}

impl PartialEq for KeyIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.device_alias == other.device_alias
    }
}
