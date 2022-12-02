use super::KeyIdentifier;

#[test]
fn can_turn_key_identifier_to_string() {
    let key = KeyIdentifier::new("L1", 32);
    assert_eq!(key.to_string(), "L1|32");

    let key = KeyIdentifier::new("R2", 44);
    assert_eq!(key.to_string(), "R2|44");
}
