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

    // rule!(L1 1 => 58) : non-modifier : map Esc to Capslock

    receive_new_fragment(&mut state, "L1", 1, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(58));

    receive_new_fragment(&mut state, "L1", 1, 0, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn consecutive_non_prefix_key_down_and_up() {
    let ruleset = mock_ruleset();

    // rule!(L1 1 => 58) : non-modifier : map Esc to Capslock

    let mut state = State::new();
    receive_new_fragment(&mut state, "L1", 1, 1, mipoch(0));
    receive_new_fragment(&mut state, "L1", 1, 0, mipoch(40));

    // rule!(L1 110 => 127) : non-modifier : map Insert to Compose

    receive_new_fragment(&mut state, "L1", 110, 1, mipoch(100));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(127));

    receive_new_fragment(&mut state, "L1", 110, 0, mipoch(150));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn modifier_key_down_and_up() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    // rule!(L1 1 => 58) : modifier : map Esc to Capslock

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(1));
}

#[test]
fn basic_combo() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    // rule!(L1 58, R1 36 => 108) : non-modifier : Capslock + J to Down

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

    // rule!(L1 29, R1 36 => 116) : modifier : Ctrl + J to VolumeDown
    // rule!(L1 29, R1 37 => 115) : modifier : Ctrl + K to VolumeUp

    receive_new_fragment(&mut state, "L1", 29, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(80));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(116));

    receive_new_fragment(&mut state, "L1", 29, 0, mipoch(120));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn chain_3_combo() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    // rule!(L1 29, R1 36, R1 37 => 26) : non-modifier : Ctrl + J + K to LeftBrace
    // rule!(L1 29, R1 37, R1 36 => 27) : non-modifier : Ctrl + K + J to RightBrace

    receive_new_fragment(&mut state, "L1", 29, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 37, 1, mipoch(90));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(26));

    receive_new_fragment(&mut state, "R1", 37, 0, mipoch(120));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(150));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "L1", 29, 0, mipoch(200));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn should_do_nothing_if_combo_not_mapped() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 88, 1, mipoch(40));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(100));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    receive_new_fragment(&mut state, "R1", 88, 0, mipoch(140));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}

#[test]
fn should_return_same_key_if_key_not_mapped() {
    let ruleset = mock_ruleset();
    let mut state = State::new();

    // code 87 : F11 key : not mapped

    receive_new_fragment(&mut state, "L1", 87, 1, mipoch(200));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(87));

    receive_new_fragment(&mut state, "L1", 87, 0, mipoch(300));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);

    // code 88 : F12 key : not mapped

    receive_new_fragment(&mut state, "L1", 88, 1, mipoch(0));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), Some(88));

    receive_new_fragment(&mut state, "L1", 88, 0, mipoch(100));
    assert_eq!(ruleset_output_to_execute(&mut state, &ruleset), None);
}
