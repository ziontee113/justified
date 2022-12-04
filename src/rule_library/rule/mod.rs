use crate::units::key_identifier::KeyIdentifier;

#[cfg(test)]
mod test;

#[macro_export]
macro_rules! rule {
    ($($a:ident $b:expr), * => $output:expr) => {
        $crate::rule_library::rule::Rule::new( vec![ $( $crate::ki!($a $b) ),* ], $output )
    };
}

#[macro_export]
/// Creates a vector of `KeyIdentifiers` from arguments
macro_rules! rulekey {
    ($($a:ident $b:expr), *) => {
        vec![ $( $crate::ki!($a $b) ),* ]
    };
}

pub struct Rule {
    input: Vec<KeyIdentifier>,
    output: u16,
}

impl Rule {
    pub fn new(input: Vec<KeyIdentifier>, output: u16) -> Self {
        Self { input, output }
    }

    pub fn input(&self) -> &[KeyIdentifier] {
        self.input.as_ref()
    }

    pub fn output(&self) -> u16 {
        self.output
    }
}
