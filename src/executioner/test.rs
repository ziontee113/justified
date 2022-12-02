use crate::interceptor::state::test::receive_new_fragment;
use crate::interceptor::state::State;
use crate::rule;
use crate::rule_library::ruleset::RuleSet;
use crate::utils::mipoch;

use super::ruleset_output_to_execute;

fn mock_ruleset() -> RuleSet {
    RuleSet::new(
        "Base Ruleset",
        vec![
            rule!(L1 58 => 1), // Capslock to Esc
            rule!(L1 1 => 58), // Esc to Capslock
            rule!(L1 29, R1 36 => 116),
            rule!(L1 29, R1 37 => 115),
        ],
    )
}

#[test]
fn can_return_ruleset_output_with_single_key_down_and_single_key_up() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&state, &ruleset), Some(1));

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&state, &ruleset), None);
}

#[test]
fn can_return_ruleset_output_with_multiple_keydowns_and_key_ups() {
    let ruleset = mock_ruleset();

    let mut state = State::new();
    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(0));
    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(40));

    receive_new_fragment(&mut state, "L1", 1, 1, mipoch(100));
    assert_eq!(ruleset_output_to_execute(&state, &ruleset), Some(58));

    receive_new_fragment(&mut state, "L1", 1, 0, mipoch(150));
    assert_eq!(ruleset_output_to_execute(&state, &ruleset), None);
}
