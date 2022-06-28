use super::*;

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
fn should_panic_number_into_bqnvalue_vec() {
    let v = BQN!("123");
    assert!(panic::catch_unwind(move || {
        v.into_bqnvalue_vec();
    })
    .is_err());
}
