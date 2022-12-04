use std::collections::{HashMap, HashSet};

use crate::units::key_identifier::KeyIdentifier;

use super::rule::Rule;

#[cfg(test)]
mod test;

pub struct RuleSet {
    rules: HashMap<Vec<KeyIdentifier>, u16>,
    name: String,
    prefixes: HashSet<Vec<KeyIdentifier>>,
}

impl RuleSet {
    pub fn new(name: &str, rules: Vec<Rule>) -> Self {
        let mut map = HashMap::new();
        let mut prefixes = HashSet::new();

        for rule in rules {
            map.insert(rule.input().into(), rule.output());
            if rule.input().len() > 1 {
                prefixes.insert(generate_prefix_from_input(rule.input()));
            }
        }

        Self {
            name: name.to_string(),
            rules: map,
            prefixes,
        }
    }

    pub fn rules(&self) -> &HashMap<Vec<KeyIdentifier>, u16> {
        &self.rules
    }

    pub fn prefixes(&self) -> &HashSet<Vec<KeyIdentifier>> {
        &self.prefixes
    }
}

fn generate_prefix_from_input(input: &[KeyIdentifier]) -> Vec<KeyIdentifier> {
    if input.len() > 1 {
        return input[0..input.len() - 1]
            .iter()
            .map(std::clone::Clone::clone)
            .collect();
    }
    input.to_vec()
}
