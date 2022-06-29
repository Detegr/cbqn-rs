use super::*;

#[test]
fn to_char() {
    let ret = eval(r#"⊑"hello""#);
    assert_eq!(ret.to_char(), Some('h'));
}

#[test]
fn to_u32() {
    let ret = eval(r#"⊑"hello""#);
    assert_eq!(ret.to_u32(), 104);
}

#[test]
fn to_f64_vec() {
    let ret = eval("2‿∘⥊↕6");
    assert_eq!(ret.to_f64_vec(), vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn to_i32_vec() {
    let ret = eval("0.75+↕5");
    assert_eq!(ret.to_i32_vec(), vec![0, 1, 2, 3, 4]);
}

#[test]
fn to_bqnvalue_vec() {
    let strings = BQN!("↑", "hello")
        .to_bqnvalue_vec()
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    assert_eq!(strings, vec!["", "h", "he", "hel", "hell", "hello"]);
}

#[test]
fn should_panic_number_to_bqnvalue_vec() {
    let v = BQN!("123");
    assert!(panic::catch_unwind(move || {
        v.to_bqnvalue_vec();
    })
    .is_err());
}
