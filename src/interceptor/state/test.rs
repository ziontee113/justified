use std::time::SystemTime;

use crate::{
    interceptor::{
        incoming_fragment::{IncomingFragment, KeyState},
        state::State,
    },
    rulekey,
    utils::mipoch,
};

fn fragment_has_contents(
    fragment: &IncomingFragment,
    device_alias: &str,
    code: u16,
    value: KeyState,
    timestamp: SystemTime,
) {
    assert_eq!(fragment.key().device_alias(), device_alias);
    assert_eq!(fragment.key().code(), code);
    assert_eq!(fragment.value(), value);
    assert_eq!(fragment.timestamp(), timestamp);
}

#[test]
fn can_add_fragment_to_state() {
    let mut state = State::new();
    let fragment = IncomingFragment::new("L1", 32, 1, mipoch(0));

    state.add_fragment(fragment);
    fragment_has_contents(
        state.sequence().get(0).unwrap(),
        "L1",
        32,
        KeyState::Down,
        mipoch(0),
    );
}

#[test]
fn can_remove_fragment_base_on_alias_and_code() {
    let mut state = State::new();

    let l1_32_down = IncomingFragment::new("L1", 32, 1, mipoch(0));
    let l1_33_down = IncomingFragment::new("L1", 33, 1, mipoch(10));
    let l1_34_down = IncomingFragment::new("L1", 34, 1, mipoch(15));
    state.add_fragment(l1_32_down);
    state.add_fragment(l1_33_down);
    state.add_fragment(l1_34_down);

    let l1_32_up = IncomingFragment::new("L1", 32, 0, mipoch(40));
    state.remove_fragment(&l1_32_up);

    assert_eq!(state.sequence().len(), 2);
    fragment_has_contents(
        state.sequence().get(0).unwrap(),
        "L1",
        33,
        KeyState::Down,
        mipoch(10),
    );
    fragment_has_contents(
        state.sequence().get(1).unwrap(),
        "L1",
        34,
        KeyState::Down,
        mipoch(15),
    );

    let l1_33_up = IncomingFragment::new("L1", 33, 0, mipoch(50));
    state.remove_fragment(&l1_33_up);

    assert_eq!(state.sequence().len(), 1);
    fragment_has_contents(
        state.sequence().get(0).unwrap(),
        "L1",
        34,
        KeyState::Down,
        mipoch(15),
    );

    let l1_34_up = IncomingFragment::new("L1", 34, 0, mipoch(60));
    state.remove_fragment(&l1_34_up);

    assert_eq!(state.sequence().len(), 0);
}

pub fn receive_new_fragment(
    state: &mut State,
    device_alias: &str,
    code: u16,
    value: i32,
    timestamp: SystemTime,
) {
    let fragment = IncomingFragment::new(device_alias, code, value, timestamp);
    state.receive(&fragment);
}

#[test]
fn can_add_or_remove_fragment_base_on_incoming_fragment_value() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 32, 1, mipoch(0));
    assert_eq!(state.sequence().len(), 1);
    fragment_has_contents(
        state.sequence().get(0).unwrap(),
        "L1",
        32,
        KeyState::Down,
        mipoch(0),
    );

    receive_new_fragment(&mut state, "L1", 33, 1, mipoch(10));
    assert_eq!(state.sequence().len(), 2);
    fragment_has_contents(
        state.sequence().get(1).unwrap(),
        "L1",
        33,
        KeyState::Down,
        mipoch(10),
    );

    receive_new_fragment(&mut state, "L1", 33, 0, mipoch(40));
    assert_eq!(state.sequence().len(), 1);
    fragment_has_contents(
        state.sequence().get(0).unwrap(),
        "L1",
        32,
        KeyState::Down,
        mipoch(0),
    );

    receive_new_fragment(&mut state, "L1", 32, 0, mipoch(58));
    assert_eq!(state.sequence().len(), 0);
}

#[test]
fn can_display_state_as_string() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 32, 1, mipoch(0));
    assert_eq!(state.to_string(), "L1|32");

    receive_new_fragment(&mut state, "L1", 33, 1, mipoch(19));
    assert_eq!(state.to_string(), "L1|32, L1|33");

    state.remove_fragment(&IncomingFragment::new("L1", 32, 0, mipoch(40)));
    assert_eq!(state.to_string(), "L1|33");

    state.remove_fragment(&IncomingFragment::new("L1", 33, 0, mipoch(53)));
    assert_eq!(state.to_string(), "");
}

#[test]
fn can_return_fragments_as_vector_of_key_identifiers() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 32, 1, mipoch(0));
    assert_eq!(state.sequence_identifiers(), rulekey!(L1 32));

    receive_new_fragment(&mut state, "L1", 32, 0, mipoch(25));
    assert_eq!(state.sequence_identifiers(), rulekey!());

    receive_new_fragment(&mut state, "L1", 33, 1, mipoch(100));
    assert_eq!(state.sequence_identifiers(), rulekey!(L1 33));
    receive_new_fragment(&mut state, "L1", 32, 1, mipoch(150));
    assert_eq!(state.sequence_identifiers(), rulekey!(L1 33, L1 32));

    receive_new_fragment(&mut state, "L1", 32, 0, mipoch(200));
    assert_eq!(state.sequence_identifiers(), rulekey!(L1 33));
    receive_new_fragment(&mut state, "L1", 33, 0, mipoch(250));
    assert_eq!(state.sequence_identifiers(), rulekey!());
}

#[test]
fn state_can_save_latest_up_down_value_and_key() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 32, 1, mipoch(0));
    assert_eq!(state.latest_value(), KeyState::Down);
    assert_eq!(i32::from(state.latest_value()), 1);
    assert!(state.latest_key.as_ref().unwrap().is("L1", 32));

    receive_new_fragment(&mut state, "L1", 32, 0, mipoch(25));
    assert_eq!(state.latest_value(), KeyState::Up);
    assert_eq!(i32::from(state.latest_value()), 0);
    assert!(state.latest_key.as_ref().unwrap().is("L1", 32));
}

#[test]
fn can_get_state_modifier_single_key_case() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(0));
    assert_eq!(state.modifier_identifiers().len(), 1);
    assert!(state.modifier_identifiers().get(0).unwrap().is("L1", 58));

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(0));
    assert_eq!(state.modifier_identifiers().len(), 0);
}

#[test]
fn can_get_state_modifiers_combo_case() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(100));
    assert_eq!(state.modifier_identifiers().len(), 1);
    assert!(state.modifier_identifiers().get(0).unwrap().is("L1", 58));

    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(150));
    assert_eq!(state.modifier_identifiers().len(), 1);
    assert!(state.modifier_identifiers().get(0).unwrap().is("L1", 58));

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(200));
    assert_eq!(state.modifier_identifiers().len(), 1);
    assert!(state.modifier_identifiers().get(0).unwrap().is("L1", 58));

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(250));
    assert_eq!(state.modifier_identifiers().len(), 0);
}

#[test]
fn can_get_identifiers_before_key_up_event() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(100));
    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(150));

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(200));
    assert_eq!(state.sequence_identifiers_before_key_up_event().len(), 2);

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(250));
    assert_eq!(state.sequence_identifiers_before_key_up_event().len(), 1);
}

#[test]
fn can_update_key_down_counter() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(100));
    assert_eq!(state.key_down_counter, 1);

    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(150));
    assert_eq!(state.key_down_counter, 2);

    receive_new_fragment(&mut state, "R1", 37, 1, mipoch(180));
    assert_eq!(state.key_down_counter, 3);

    receive_new_fragment(&mut state, "R1", 37, 0, mipoch(200));
    assert_eq!(state.key_down_counter, 2);

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(220));
    assert_eq!(state.key_down_counter, 1);

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(250));
    assert_eq!(state.key_down_counter, 0);
}

#[test]
fn can_update_key_down_combo_count() {
    let mut state = State::new();

    receive_new_fragment(&mut state, "L1", 58, 1, mipoch(100));
    assert_eq!(state.key_down_combo_count(), 1);

    receive_new_fragment(&mut state, "R1", 36, 1, mipoch(150));
    assert_eq!(state.key_down_combo_count(), 2);

    receive_new_fragment(&mut state, "R1", 37, 1, mipoch(180));
    assert_eq!(state.key_down_combo_count(), 3);

    receive_new_fragment(&mut state, "R1", 37, 0, mipoch(200));
    assert_eq!(state.key_down_combo_count(), 3);

    receive_new_fragment(&mut state, "R1", 36, 0, mipoch(220));
    assert_eq!(state.key_down_combo_count(), 3);

    receive_new_fragment(&mut state, "L1", 58, 0, mipoch(250));
    assert_eq!(state.key_down_combo_count(), 3);
}
