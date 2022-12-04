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

#[test]
fn non_prefix_key_down_and_up() {
    receive_and_assert(
        &mut State::new(),
        &mock_ruleset(),
        vec![
            tsc!(L1 ESC, 1, 0 => Some(58), "wanted 58"),
            tsc!(L1 ESC, 0, 100 => None, "didn't expect to see result"),
        ],
    );
}
