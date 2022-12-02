use crate::{rule, rulekey};

use super::{generate_prefix_from_input, RuleSet};

#[test]
fn can_create_new_ruleset_from_rules() {
    let ruleset = RuleSet::new(
        "Base Ruleset",
        vec![
            rule!(L1 58 => 1),
            rule!(L1 29, R1 36 => 116),
            rule!(L1 29, R1 37 => 115),
        ],
    );

    let first_key = rulekey!(L1 58);
    assert_eq!(*ruleset.rules.get(&first_key).unwrap(), 1);

    let second_key = rulekey!(L1 29, R1 36);
    assert_eq!(*ruleset.rules.get(&second_key).unwrap(), 116);

    let third_key = rulekey!(L1 29, R1 37);
    assert_eq!(*ruleset.rules.get(&third_key).unwrap(), 115);
}

#[test]
fn can_generate_key_prefix_from_rule_input() {
    let rule = rule!(L1 58 => 1);
    let prefix = generate_prefix_from_input(rule.input());
    assert_eq!(prefix.len(), 1);
    assert!(prefix.get(0).unwrap().is("L1", 58));

    let rule = rule!(L1 58, R1 44 => 1);
    let prefix = generate_prefix_from_input(rule.input());
    assert_eq!(prefix.len(), 1);
    assert!(prefix.get(0).unwrap().is("L1", 58));

    let rule = rule!(L1 58, R1 44, R2 55 => 1);
    let prefix = generate_prefix_from_input(rule.input());
    assert_eq!(prefix.len(), 2);
    assert!(prefix.get(0).unwrap().is("L1", 58));
    assert!(prefix.get(1).unwrap().is("R1", 44));
}

#[test]
fn can_generate_prefix_hash_set_when_creating_ruleset() {
    let ruleset = RuleSet::new(
        "Base Ruleset",
        vec![
            rule!(L1 58 => 1),
            rule!(L1 29, R1 36 => 116),
            rule!(L1 29, R1 36, R1 37 => 1),
            rule!(L1 29, R1 37 => 115),
            rule!(L1 29, R1 37, R1 36 => 1),
        ],
    );
    let prefixes = ruleset.prefixes();
    assert_eq!(prefixes.len(), 3);
    assert!(prefixes.contains(&rulekey!(L1 29)));
    assert!(prefixes.contains(&rulekey!(L1 29, R1 36)));
    assert!(prefixes.contains(&rulekey!(L1 29, R1 37)));
}
