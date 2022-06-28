use crate::*;
use std::panic;

fn c8_str() -> BQNValue {
    BQN!(r#""hello""#)
}

fn c16_str() -> BQNValue {
    BQN!(r#""hello∘""#)
}

fn c32_str() -> BQNValue {
    BQN!(r#""hello💣""#)
}

fn ns() -> BQNValue {
    BQN!("{a⇐1, B⇐{a+𝕩}}")
}

#[test]
fn into_char() {
    let ret = eval(r#"⊑"hello""#);
    assert_eq!(ret.into_char(), Some('h'));
}

#[test]
fn into_u32() {
    let ret = eval(r#"⊑"hello""#);
    assert_eq!(ret.into_u32(), 104);
}

#[test]
fn into_f64_vec() {
    let ret = eval("2‿∘⥊↕6");
    assert_eq!(ret.into_f64_vec(), vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn into_i32_vec() {
    let ret = eval("0.25+↕5");
    assert_eq!(ret.into_i32_vec(), vec![0, 1, 2, 3, 4]);
}

#[test]
fn into_bqnvalue_vec() {
    let strings = BQN!("↑", "hello")
        .into_bqnvalue_vec()
        .into_iter()
        .map(|v| v.into_string())
        .collect::<Vec<String>>();
    assert_eq!(strings, vec!["", "h", "he", "hel", "hell", "hello"]);
}

#[test]
fn call1() {
    let f = eval("↕");
    let ret = f.call1(&5.into());
    assert_eq!(ret.into_i32_vec(), vec![0, 1, 2, 3, 4]);
}

#[test]
fn call2() {
    let f = eval("⊑");
    let ret = f.call2(&3.into(), &"hello".into());
    assert_eq!(ret.into_char(), Some('l'));
}

#[test]
fn fixed_size_array() {
    let f = eval("+´");
    f.call1(&[0.0, 1.0, 2.0, 3.0, 4.0].into());
}

#[test]
fn from_iterator_f64() {
    let f = eval("+´");
    let ret = f.call1(&BQNValue::from_iter(
        [0.0f64, 1.0, 2.0, 3.0, 4.0].into_iter(),
    ));
    assert_eq!(ret.into_f64(), 10.0);
}

#[test]
fn from_iterator_i32() {
    let f = eval("+´");
    let ret = f.call1(&BQNValue::from_iter((0i32..).take(5)));
    assert_eq!(ret.into_f64(), 10.0);
}

#[test]
fn from_iterator_i16() {
    let f = eval("+´");
    let ret = f.call1(&(0i16..).take(5).collect::<BQNValue>());
    assert_eq!(ret.into_f64(), 10.0);
}

#[test]
fn from_iterator_i8() {
    let f = eval("+´");
    let ret = f.call1(&BQNValue::from_iter((0i8..).take(5)));
    assert_eq!(ret.into_f64(), 10.0);
}

#[test]
fn from_char() {
    let f = eval("+⟜1");
    let ret = f.call1(&'a'.into());
    assert_eq!(ret.into_char(), Some('b'));
}

#[test]
fn from_slice() {
    let f = eval("+´");
    let ret = f.call1(&[1i32, 2, 3, 4, 5].as_slice().into());
    assert_eq!(ret.into_f64(), 15.0);
}

#[test]
fn bqn_macro() {
    assert_eq!(BQN!("+´", [1, 2, 3]).into_f64(), 6.0);
    assert_eq!(BQN!('a', "+", 1).into_char(), Some('b'));
    let arr = BQN!("+`", [1, 2, 3]);
    assert_eq!(BQN!(2, "×", arr).into_i32_vec(), vec![2, 6, 12]);
}

#[test]
fn test_debug_repr() {
    let v = BQN!("1‿2‿3");
    assert_eq!(format!("{:?}", v), "⟨ 1 2 3 ⟩");
}

#[test]
fn should_panic_c8_string_to_f64_vec() {
    let v = c8_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
    assert!(panic::catch_unwind(move || {
        _ = v.into_f64_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c16_string_to_f64_vec() {
    let v = c16_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
    assert!(panic::catch_unwind(move || {
        _ = v.into_f64_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c32_string_to_f64_vec() {
    let v = c32_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
    assert!(panic::catch_unwind(move || {
        _ = v.into_f64_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c8_string_to_i32_vec() {
    let v = c8_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
    assert!(panic::catch_unwind(move || {
        _ = v.into_i32_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c16_string_to_i32_vec() {
    let v = c16_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
    assert!(panic::catch_unwind(move || {
        _ = v.into_i32_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c32_string_to_i32_vec() {
    let v = c32_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
    assert!(panic::catch_unwind(move || {
        v.into_i32_vec();
    })
    .is_err());
}

#[test]
fn should_panic_f64_arr_to_string() {
    let v = BQN!("1.2‿3.4‿5.6");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_f64);
    assert!(panic::catch_unwind(move || {
        let _ = v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_i32_arr_to_string() {
    let v = BQN!("67000‿68000");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i32);
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_i16_arr_to_string() {
    let v = BQN!("1234‿5678");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i16);
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_i8_arr_to_string() {
    let v = BQN!("12‿34");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i8);
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_number_to_string() {
    let v = BQN!("123");
    assert!(panic::catch_unwind(move || {
        v.into_string();
    })
    .is_err());
}

#[test]
fn should_panic_c8_string_to_bqnvalue_vec() {
    let v = c8_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
    assert!(panic::catch_unwind(move || {
        _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c16_string_to_bqnvalue_vec() {
    let v = c16_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
    assert!(panic::catch_unwind(move || {
        _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_c32_string_to_bqnvalue_vec() {
    let v = c32_str();
    assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
    assert!(panic::catch_unwind(move || {
        _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_f64_arr_to_bqnvalue_vec() {
    let v = BQN!("1.2‿3.4‿5.6");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_f64);
    assert!(panic::catch_unwind(move || {
        let _ = v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_i32_arr_to_bqnvalue_vec() {
    let v = BQN!("67000‿68000");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i32);
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_i16_arr_to_bqnvalue_vec() {
    let v = BQN!("1234‿5678");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i16);
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_i8_arr_to_bqnvalue_vec() {
    let v = BQN!("12‿34");
    assert_eq!(v.direct_arr_type(), BQNElType_elt_i8);
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
fn should_panic_number_to_bqnvalue_vec() {
    let v = BQN!("123");
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}

#[test]
#[should_panic]
fn should_panic_null_to_f64() {
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
fn null_to_char() {
    let _ = BQNValue::null().into_char();
}

#[test]
fn null_to_u32() {
    let _ = BQNValue::null().into_u32();
}

#[test]
#[should_panic]
fn should_panic_null_to_f64_vec() {
    let _ = BQNValue::null().into_f64_vec();
}

#[test]
#[should_panic]
fn should_panic_null_to_i32_vec() {
    let _ = BQNValue::null().into_i32_vec();
}

#[test]
#[should_panic]
fn should_panic_null_to_bqnvalue_vec() {
    let _ = BQNValue::null().into_bqnvalue_vec();
}

#[test]
#[should_panic]
fn should_panic_null_to_string() {
    let _ = BQNValue::null().into_string();
}

#[test]
#[should_panic]
fn should_panic_null_to_char_vec() {
    let _ = BQNValue::null().into_char_vec();
}

#[test]
fn namespace() {
    let ns = ns();

    assert!(ns.has_field("a"));
    assert!(ns.get_field("a").is_some());
    assert!(!ns.has_field("A"));
    assert!(ns.get_field("A").is_none());
    assert!(ns.has_field("b"));
    assert!(ns.get_field("b").is_some());
    assert!(!ns.has_field("B"));
    assert!(ns.get_field("B").is_none());
    assert!(!ns.has_field("c"));
    assert!(ns.get_field("c").is_none());

    assert_eq!(ns.get_field("a").map(BQNValue::into_f64), Some(1.0));
    assert_eq!(
        ns.get_field("b").map(|b| b.call1(&1.into()).into_f64()),
        Some(2.0)
    );
}
