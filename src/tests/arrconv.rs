use super::*;

#[test]
fn should_panic_c8_string_into_f64_vec() {
    let v = c8_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
    assert!(panic::catch_unwind(move || {
        _ = v.into_f64_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c16_string_into_f64_vec() {
    let v = c16_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
    assert!(panic::catch_unwind(move || {
        _ = v.into_f64_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c32_string_into_f64_vec() {
    let v = c32_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
    assert!(panic::catch_unwind(move || {
        _ = v.into_f64_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c8_string_into_i32_vec() {
    let v = c8_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
    assert!(panic::catch_unwind(move || {
        _ = v.into_i32_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c16_string_into_i32_vec() {
    let v = c16_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
    assert!(panic::catch_unwind(move || {
        _ = v.into_i32_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c32_string_into_i32_vec() {
    let v = c32_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
    assert!(panic::catch_unwind(move || {
        v.into_i32_vec();
    })
    .is_err());
}

#[test]
fn should_panic_f64_arr_into_string() {
    let v = BQN!("1.2‿3.4‿5.6");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_f64);
    assert!(panic::catch_unwind(move || {
        let _ = v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_i32_arr_into_string() {
    let v = BQN!("67000‿68000");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i32);
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_i16_arr_into_string() {
    let v = BQN!("1234‿5678");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i16);
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_i8_arr_into_string() {
    let v = BQN!("12‿34");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i8);
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_number_into_string() {
    let v = BQN!("123");
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_c8_string_into_bqnvalue_vec() {
    let v = c8_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
    assert!(panic::catch_unwind(move || {
        _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c16_string_into_bqnvalue_vec() {
    let v = c16_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
    assert!(panic::catch_unwind(move || {
        _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c32_string_into_bqnvalue_vec() {
    let v = c32_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
    assert!(panic::catch_unwind(move || {
        _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_f64_arr_into_bqnvalue_vec() {
    let v = BQN!("1.2‿3.4‿5.6");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_f64);
    assert!(panic::catch_unwind(move || {
        let _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_i32_arr_into_bqnvalue_vec() {
    let v = BQN!("67000‿68000");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i32);
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_i16_arr_into_bqnvalue_vec() {
    let v = BQN!("1234‿5678");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i16);
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_i8_arr_into_bqnvalue_vec() {
    let v = BQN!("12‿34");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i8);
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}
