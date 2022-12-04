use crate::interceptor::state::test::receive_new_fragment;
use crate::interceptor::state::State;
use crate::rule;
use crate::rule_library::ruleset::RuleSet;
use crate::units::key_code::code_from_key;
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

struct TestCase<T: AsRef<str>> {
    device_alias: T,
    code: u16,
    value: i32,
    millis: u64,

    want: Option<u16>,
    message: T,
}

impl<T: AsRef<str>> TestCase<T> {
    fn new(
        device_alias: T,
        str_code: &str,
        value: i32,
        millis: u64,
        want: Option<u16>,
        message: T,
    ) -> Self {
        Self {
            device_alias,
            code: code_from_key(str_code).unwrap(),
            value,
            millis,
            want,
            message,
        }
    }
}

fn receive_and_assert<T: AsRef<str>>(state: &mut State, ruleset: &RuleSet, tcs: Vec<TestCase<T>>) {
    for tc in tcs {
        receive_new_fragment(
            state,
            tc.device_alias.as_ref(),
            tc.code,
            tc.value,
            mipoch(tc.millis),
        );
        assert_eq!(
            ruleset_output_to_execute(state, ruleset),
            tc.want,
            "{}",
            tc.message.as_ref()
        );
    }
}

macro_rules! tsc {
    ($device_alias:ident $str_code:ident, $value:expr, $millis:expr => $want:expr, $message:expr) => {
        TestCase::new(
            stringify!($device_alias),
            stringify!($str_code),
            $value,
            $millis,
            $want,
            $message,
        )
    };
}

// Aliases:
// 1. mono --> non-modifier key
// 2. mono-combo --> combo is not modifier of another combo
// 3. modifer --> modifier key
// 4. modifer-combo --> modifier combo that is modifier of another combo

#[test]
fn mono_key_down_and_up() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 ESC, 1, 0 => Some(58), "esc down, mono, wanted 58"),
            tsc!(L1 ESC, 0, 50 => None, "esc up, mono, expected none"),
        ],
    );
}

#[test]
fn consecutive_mono_key_down_and_up() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 ESC, 1, 0 => Some(58), "esc down, mono, wanted 58 (Capslock)"),
            tsc!(L1 ESC, 0, 50 => None, "esc up, mono, expected none"),
            tsc!(L1 INSERT, 1, 100 => Some(127), "insert down, mono, wanted 127 (Compose)"),
            tsc!(L1 INSERT, 0, 150 => None, "insert up, mono, expected none"),
        ],
    );
}

#[test]
fn modifier_key_down_and_up() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 CAPSLOCK, 1, 0 => None, "Capslock down, modifer, expected none"),
            tsc!(L1 CAPSLOCK, 0, 50 => Some(1), "Capslock up, modifier, wanted 1 (Escape)"),
        ],
    );
}

#[test]
fn mono_combo() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 CAPSLOCK, 1, 0 => None, "Caps down, modifier, expected none"),
            tsc!(R1 J, 1, 50 => Some(108), "J down, mono-combo, wanted 108 (Down)"),
            tsc!(R1 J, 0, 100 => None, "J up, mono-combo, expected none"),
            tsc!(L1 CAPSLOCK, 0, 150 => None, "Caps up, modifier, expected none"),
        ],
    );
}

#[test]
fn consecutive_mono_combos() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 CAPSLOCK, 1, 0 => None, "Caps down, modifier, expected none"),
            tsc!(R1 J, 1, 50 => Some(108), "J down, mono-combo, wanted 108 (Down)"),
            tsc!(R1 J, 0, 100 => None, "J up, mono-combo, expected none"),
            tsc!(R1 K, 1, 150 => Some(103), "K down, mono-combo, wanted 103 (Up)"),
            tsc!(R1 K, 0, 200 => None, "K up, mono-combo, expected none"),
            tsc!(L1 CAPSLOCK, 0, 400 => None, "Caps up, modifier, expected none"),
        ],
    );
}

#[test]
fn chain_2_modifier_combo() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 LEFTCTRL, 1, 0 => None, "LeftCtrl down, modifier, expected none"),
            tsc!(R1 J, 1, 50 => None, "J down, modifier-combo, expected none"),
            tsc!(R1 J, 0, 100 => Some(116), "J down, modifier-combo, wanted 116 (VolumeDown)"),
            tsc!(L1 LEFTCTRL, 0, 200 => None, "LeftCtrl up, modifier, expected none"),
        ],
    );
}

#[test]
fn consecutive_chain_2_modifier_combos() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 LEFTCTRL, 1, 0 => None, "LeftCtrl down, modifier, expected none"),
            tsc!(R1 J, 1, 50 => None, "J down, modifier-combo, expected none"),
            tsc!(R1 J, 0, 100 => Some(116), "J down, modifier-combo, wanted 116 (VolumeDown)"),
            tsc!(R1 K, 1, 150 => None, "K down, modifier-combo, expected none"),
            tsc!(R1 K, 0, 200 => Some(115), "K down, modifier-combo, wanted 115 (VolumeUp)"),
            tsc!(L1 LEFTCTRL, 0, 300 => None, "LeftCtrl up, modifier, expected none"),
        ],
    );
}

#[test]
fn chain_3_modifier_combos() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 LEFTCTRL, 1, 0 => None, "LeftCtrl down, modifier, expected none"),
            tsc!(R1 J, 1, 50 => None, "J down, modifier-combo, expected none"),
            tsc!(R1 K, 1, 100 => Some(26), "K down, mono-combo, wanted 26 (RightBrace)"),
            tsc!(R1 K, 0, 150 => None, "K up, mono-combo, expected none"),
            tsc!(R1 J, 0, 200 => None, "J up, modifier-combo, expected none"),
            tsc!(L1 LEFTCTRL, 0, 250 => None, "LeftCtrl up, modifier, expected none"),
        ],
    );
}

#[test]
fn consecutive_chain_3_modifier_combos() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 LEFTCTRL, 1, 0 => None, "LeftCtrl down, modifier, expected none"),
            // first round start
            tsc!(R1 J, 1, 50 => None, "C-J down, modifier-combo, expected none"),
            tsc!(R1 K, 1, 100 => Some(26), "C-J-K down, mono-combo, wanted 26 (LeftBrace)"),
            tsc!(R1 K, 0, 150 => None, "K up, mono-combo, expected none"),
            tsc!(R1 J, 0, 200 => None, "J up, modifier-combo, expected none"),
            // first round end, second round start
            tsc!(R1 K, 1, 250 => None, "C-K down, modifier-combo, expected none"),
            tsc!(R1 J, 1, 300 => Some(27), "C-K-J down, mono-combo, wanted 27 (RightBrace)"),
            tsc!(R1 J, 0, 350 => None, "J up, modifier-combo, expected none"),
            tsc!(R1 K, 0, 400 => None, "K up, mono-combo, expected none"),
            // second round end
            tsc!(L1 LEFTCTRL, 0, 500 => None, "LeftCtrl up, modifier, expected none"),
        ],
    );
}

#[test]
fn return_same_key_if_key_not_mapped() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            // F11 not mapped
            tsc!(L1 F11, 1, 0 => Some(87), "F11 down, not mapped, wanted 87 (F11)"),
            tsc!(L1 F11, 0, 100 => None, "F11 up, not mapped, expected none"),
            // F12 not mapped
            tsc!(L1 F12, 1, 150 => Some(88), "F12 down, not mapped, wanted 88 (F12)"),
            tsc!(L1 F12, 0, 200 => None, "F12 up, not mapped, expected none"),
        ],
    );
}

// ---------------------------------------------------------------------- Key Hold

#[test]
fn repeat_basic_combo_on_key_hold() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 Capslock, 1, 0 => None, "Capslock down, modifier, expected none"),
            tsc!(R1 J, 1, 0 => Some(108), "J down, mono-combo, expected 108 (Down)"),
            tsc!(R1 J, 2, 0 => Some(108), "J hold, mono-combo, expected 108 (Down)"),
            tsc!(R1 J, 2, 0 => Some(108), "J hold, mono-combo, expected 108 (Down)"),
            tsc!(R1 J, 2, 0 => Some(108), "J hold, mono-combo, expected 108 (Down)"),
            tsc!(L1 Capslock, 0, 400 => None, "Capslock up, modifier, expected none"),
        ],
    );
}

#[test]
fn repeat_same_key_if_key_not_mapped_on_key_hold() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 F11, 1, 0 => Some(87), "F11 down, not mapped, wanted 87 (F11)"),
            tsc!(L1 F11, 2, 50 => Some(87), "F11 hold, not mapped, wanted 87 (F11)"),
            tsc!(L1 F11, 2, 100 => Some(87), "F11 hold, not mapped, wanted 87 (F11)"),
            tsc!(L1 F11, 0, 200 => None, "F11 up, not mapped, expected none"),
        ],
    );
}
