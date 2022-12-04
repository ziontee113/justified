use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use crate::rule;
use crate::rule_library::ruleset::RuleSet;

/// Returns a timestamp elasped `milis` milliseconds from UNIX EPOCH.
/// For easy testing purposes only.
pub fn mipoch(milis: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_millis(milis)
}

// for development purposes only
pub fn mock_device_alias() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
    ])
}

pub fn mock_ruleset() -> RuleSet {
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
