use super::code_from_key;

#[test]
fn can_turn_str_to_key_code() {
    assert_eq!(code_from_key("ESC"), Some(1));
    assert_eq!(code_from_key("j"), Some(36));
    assert_eq!(code_from_key("J"), Some(36));
}
