use super::Rule;
use crate::units::KeyIdentifier;

#[test]
fn can_create_new_rule_with_single_input_and_single_output() {
    let rule = Rule::new(KeyIdentifier::new("L1", 32), 115);

    assert_eq!(rule.input.device_alias(), "L1");
    assert_eq!(rule.input.code(), 32);
    assert_eq!(rule.output, 115);
}
