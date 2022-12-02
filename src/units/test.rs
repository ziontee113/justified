use crate::ki;

#[test]
fn can_use_macro_to_create_key_identifier() {
    let key = ki!(L1 32);
    assert_eq!(key.to_string(), "L1|32");

    let key = ki!(R2 44);
    assert_eq!(key.to_string(), "R2|44");
}
