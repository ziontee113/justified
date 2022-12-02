use std::collections::HashMap;

use crate::units::KeyIdentifier;

use super::rule::Rule;

#[cfg(test)]
mod test;

pub struct RuleSet {
    rules: HashMap<Vec<KeyIdentifier>, u16>,
}

impl RuleSet {
    pub fn new(rules: Vec<Rule>) -> Self {
        let mut map = HashMap::new();

        for rule in rules {
            map.insert(rule.input().into(), rule.output());
        }

        Self { rules: map }
    }
}
