use crate::units::KeyIdentifier;

#[cfg(test)]
mod test;

#[macro_export]
macro_rules! rule {
    ($($a:ident $b:expr), * => $output:expr) => {
        $crate::rule_library::rule::Rule::new( vec![ $(ki!($a $b)),* ], $output )
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
}
