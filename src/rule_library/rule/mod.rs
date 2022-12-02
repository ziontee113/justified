use crate::units::KeyIdentifier;

#[cfg(test)]
mod test;

pub struct Rule {
    input: KeyIdentifier,
    output: u16,
}

impl Rule {
    pub fn new(input: KeyIdentifier, output: u16) -> Self {
        Self { input, output }
    }
}
