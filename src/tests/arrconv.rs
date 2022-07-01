use super::*;

#[test]
#[should_panic]
fn c8_string_to_f64_vec() {
    let v = c8_str();
    _ = v.to_f64();
}

#[test]
#[should_panic]
fn c16_string_to_f64_vec() {
    let v = c16_str();
    _ = v.to_f64();
}

#[test]
#[should_panic]
fn c32_string_to_f64_vec() {
    let v = c32_str();
    _ = v.to_f64_vec();
}

#[test]
#[should_panic]
fn should_panic_f64_arr_to_string() {
    let v = BQN!("1.2‿3.4‿5.6");
    let _ = v.to_string();
}

#[test]
#[should_panic]
fn i32_arr_to_string() {
    let v = BQN!("67000‿68000");
    let _ = v.to_string();
}

#[test]
#[should_panic]
fn should_panic_i16_arr_to_string() {
    let v = BQN!("1234‿5678");
    let _ = v.to_string();
}

#[test]
#[should_panic]
fn i8_arr_to_string() {
    let v = BQN!("12‿34");
    let _ = v.to_string();
}

#[test]
#[should_panic]
fn should_panic_number_to_string() {
    let v = BQN!("123");
    let _ = v.to_string();
}

#[test]
fn c8_string_to_bqnvalue_vec() {
    assert_eq!(
        c8_str()
            .to_bqnvalue_vec()
            .iter()
            .map(|c| c.to_char().unwrap())
            .collect::<String>(),
        c8_str().to_string()
    );
}

#[test]
fn c16_string_to_bqnvalue_vec() {
    assert_eq!(
        c16_str()
            .to_bqnvalue_vec()
            .iter()
            .map(|c| c.to_char().unwrap())
            .collect::<String>(),
        c16_str().to_string()
    );
}

#[test]
fn c32_string_to_bqnvalue_vec() {
    assert_eq!(
        c32_str()
            .to_bqnvalue_vec()
            .iter()
            .map(|c| c.to_char().unwrap())
            .collect::<String>(),
        c32_str().to_string()
    );
}

#[test]
fn arr_to_bqnvalue_vec() {
    assert_eq!(
        BQN!("12‿34")
            .to_bqnvalue_vec()
            .iter()
            .map(BQNValue::to_f64)
            .collect::<Vec<_>>(),
        [12.0, 34.0]
    );
}
