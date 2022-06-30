use super::*;

#[test]
#[should_panic]
fn should_panic_null_to_f64() {
    let _ = BQNValue::null().to_f64();
}

#[test]
#[should_panic]
fn should_panic_null_get_field() {
    let _ = BQNValue::null().get_field("test");
}

#[test]
#[should_panic]
fn should_panic_null_has_field() {
    let _ = BQNValue::null().has_field("test");
}

#[test]
fn null_to_char() {
    let _ = BQNValue::null().to_char();
}

#[test]
fn null_to_u32() {
    let _ = BQNValue::null().to_u32();
}

#[test]
#[should_panic]
fn should_panic_null_to_f64_vec() {
    let _ = BQNValue::null().to_f64_vec();
}

#[test]
#[should_panic]
fn should_panic_null_to_bqnvalue_vec() {
    let _ = BQNValue::null().to_bqnvalue_vec();
}

#[test]
#[should_panic]
fn should_panic_null_to_string() {
    let _ = BQNValue::null().to_string();
}

#[test]
#[should_panic]
fn should_panic_null_to_char_vec() {
    let _ = BQNValue::null().to_char_vec();
}
