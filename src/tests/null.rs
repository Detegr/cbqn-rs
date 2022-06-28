use super::*;

#[test]
#[should_panic]
fn should_panic_null_into_f64() {
    let _ = BQNValue::null().into_f64();
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
fn null_into_char() {
    let _ = BQNValue::null().into_char();
}

#[test]
fn null_into_u32() {
    let _ = BQNValue::null().into_u32();
}

#[test]
#[should_panic]
fn should_panic_null_into_f64_vec() {
    let _ = BQNValue::null().into_f64_vec();
}

#[test]
#[should_panic]
fn should_panic_null_into_i32_vec() {
    let _ = BQNValue::null().into_i32_vec();
}

#[test]
#[should_panic]
fn should_panic_null_into_bqnvalue_vec() {
    let _ = BQNValue::null().into_bqnvalue_vec();
}

#[test]
#[should_panic]
fn should_panic_null_into_string() {
    let _ = BQNValue::null().into_string();
}

#[test]
#[should_panic]
fn should_panic_null_into_char_vec() {
    let _ = BQNValue::null().into_char_vec();
}
