use crate::{
    interceptor::incoming_fragment::{IncomingFragment, KeyState},
    utils::mipoch,
};

#[test]
fn can_make_new_fragment() {
    let fragment = IncomingFragment::new("L1", 32, 1, mipoch(0));
    assert_eq!(fragment.key.device_alias(), "L1");
    assert_eq!(fragment.key.code(), 32);
    assert_eq!(fragment.value(), KeyState::Down);
    assert_eq!(fragment.timestamp(), mipoch(0));
}

#[test]
fn can_turn_fragment_to_string() {
    let fragment = IncomingFragment::new("L1", 32, 1, mipoch(0));
    assert_eq!(fragment.to_string(), "L1|32");

    let fragment = IncomingFragment::new("R2", 44, 0, mipoch(12));
    assert_eq!(fragment.to_string(), "R2|44");
}

#[test]
fn can_check_if_another_fragment_has_same_key_as_this_one() {
    let before = IncomingFragment::new("L1", 32, 1, mipoch(0));
    let after = IncomingFragment::new("L1", 32, 0, mipoch(49));

    assert!(before.has_same_key(&after));
}
