use crate::{rule, rulekey};

use super::RuleSet;

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
