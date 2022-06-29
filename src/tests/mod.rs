mod arrconv;
mod boundfn;
mod from;
mod gen;
mod null;
mod to;

use crate::*;
use gen::*;
use std::panic;

#[test]
fn call1() {
    let f = eval("↕");
    let ret = f.call1(&5.into());
    assert_eq!(ret.to_i32_vec(), vec![0, 1, 2, 3, 4]);
}

#[test]
fn call2() {
    let f = eval("⊑");
    let ret = f.call2(&3.into(), &"hello".into());
    assert_eq!(ret.to_char(), Some('l'));
}

#[test]
fn fixed_size_array() {
    let f = eval("+´");
    f.call1(&[0.0, 1.0, 2.0, 3.0, 4.0].into());
}

#[test]
fn bqn_macro() {
    assert_eq!(BQN!("+´", [1, 2, 3]).to_f64(), 6.0);
    assert_eq!(BQN!('a', "+", 1).to_char(), Some('b'));
    let arr = BQN!("+`", [1, 2, 3]);
    assert_eq!(BQN!(2, "×", arr).to_i32_vec(), vec![2, 6, 12]);
}

#[test]
fn test_debug_repr() {
    let v = BQN!("1‿2‿3");
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
        ns.get_field("b").map(|b| b.call1(&1.into()).to_f64()),
        Some(2.0)
    );
}
