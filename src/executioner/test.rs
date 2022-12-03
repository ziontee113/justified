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
            rule!(L1 1 => 58),                // map Esc to Capslock: non-combo
            rule!(L1 110 => 127),             // map Insert to Compose: non-combo
            rule!(L1 58 => 1),                // map Capslock to Esc: combo-prefix
            rule!(L1 58, R1 36 => 108),       // Capslock + J to Down: combo
            rule!(L1 58, R1 37 => 103),       // Capslock + K to Up: combo
            rule!(L1 29, R1 36 => 116),       // Ctrl + J to VolumeDown
            rule!(L1 29, R1 37 => 115),       // Ctrl + K to VolumeUp
            rule!(L1 29, R1 36, R1 37 => 26), // Ctrl + J + K to LeftBrace
            rule!(L1 29, R1 37, R1 36 => 27), // Ctrl + K + J to RightBrace
        ],
    )
}

#[test]
fn non_prefix_key_down_and_up() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 1, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(58));

    receive_new_fragment(&mut state, "L1", 1, 0, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn consecutive_non_prefix_key_down_and_up() {
    let ruleset = mock_ruleset();

    let mut state = State::new();
    receive_new_fragment(&mut state, "L1", 1, 1, mipoch(0));
    receive_new_fragment(&mut state, "L1", 1, 0, mipoch(40));

    receive_new_fragment(&mut state, "L1", 110, 1, mipoch(100));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(127));

    receive_new_fragment(&mut state, "L1", 110, 0, mipoch(150));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn modifier_key_down_and_up() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(1));
}

#[test]
fn basic_combo() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(108));

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(80));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(120));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn chain_2_combo() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 29, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(80));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(116));

    receive_new_fragment(&mut state, "L1", 29, 0, mipoch(120));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}
