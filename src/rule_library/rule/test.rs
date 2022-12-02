use super::Rule;
use crate::units::KeyIdentifier;

#[test]
fn create_new_rule_with_multiple_input_and_single_output() {
    let rule = Rule::new(
        vec![KeyIdentifier::new("L1", 29), KeyIdentifier::new("R1", 36)],
        115,
    );

    assert_eq!(rule.input.get(0).unwrap().device_alias(), "L1");
    assert_eq!(rule.input.get(0).unwrap().code(), 29);
    assert_eq!(rule.input.get(1).unwrap().device_alias(), "R1");
    assert_eq!(rule.input.get(1).unwrap().code(), 36);
    assert_eq!(rule.output, 115);
}
