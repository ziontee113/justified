use crate::{ki, rule};

#[test]
fn can_create_new_rule_with_multiple_inputs_and_single_output_using_macro() {
    let rule = rule!(L1 29, R1 36 => 115);

    assert_eq!(rule.input.get(0).unwrap().device_alias(), "L1");
    assert_eq!(rule.input.get(0).unwrap().code(), 29);
    assert_eq!(rule.input.get(1).unwrap().device_alias(), "R1");
    assert_eq!(rule.input.get(1).unwrap().code(), 36);
    assert_eq!(rule.output, 115);
}
