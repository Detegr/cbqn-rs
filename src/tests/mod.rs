mod arrconv;
#[cfg(not(feature = "wasi-backend"))]
mod boundfn;
mod error;
mod from;
mod gen;
mod null;
mod to;

use crate::*;
use gen::*;
use std::panic;

#[test]
fn call1() {
    let f = eval("↕").unwrap();
    let ret = f.call1(&5.into()).unwrap();
    assert_eq!(ret.to_f64_vec(), vec![0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn call2() {
    let f = eval("⊑").unwrap();
    let ret = f.call2(&3.into(), &"hello".into()).unwrap();
    assert_eq!(ret.to_char(), Some('l'));
}

#[test]
fn fixed_size_array() {
    let f = eval("+´").unwrap();
    let ret = f.call1(&[0.0, 1.0, 2.0, 3.0, 4.0].into()).unwrap();
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn bqn_macro() {
    assert_eq!(BQN!("3").unwrap().to_f64(), 3.0);
    assert_eq!(BQN!("+´", [1, 2, 3]).unwrap().to_f64(), 6.0);
    assert_eq!(BQN!('a', "+", 1).unwrap().to_char(), Some('b'));
    let arr = BQN!("+`", [1, 2, 3]).unwrap();
    assert_eq!(
        BQN!(2, "×", arr).unwrap().to_f64_vec(),
        vec![2.0, 6.0, 12.0]
    );
}

#[test]
fn test_debug_repr() {
    let v = BQN!("1‿2‿3").unwrap();
    assert_eq!(format!("{:?}", v), "⟨ 1 2 3 ⟩");
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

    assert_eq!(ns.get_field("a").as_ref().map(BQNValue::to_f64), Some(1.0));
    assert_eq!(
        ns.get_field("b")
            .map(|b| b.call1(&1.into()).unwrap().to_f64()),
        Some(2.0)
    );
}

#[test]
fn clone() {
    let v = BQNValue::from("hello");
    assert_eq!(BQN!(v.clone(), "≡", v).unwrap().to_f64(), 1.0);
}
