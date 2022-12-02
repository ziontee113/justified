#[derive(Debug, Clone)]
pub struct KeyIdentifier {
    device_alias: String,
    code: u16,
}

impl KeyIdentifier {
    pub fn new(device_alias: String, code: u16) -> Self {
        Self { device_alias, code }
    }

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
