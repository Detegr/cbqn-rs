use super::*;

#[test]
fn from_iterator_bqnvalue() {
    let fns = "≠≢"
        .chars()
        .map(|c| eval(&c.to_string()))
        .collect::<BQNValue>();

    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_bqnvalue_vec() {
    let fns = vec![eval("≠"), eval("≢")];
    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_bqnvalue_array() {
    let fns = [eval("≠"), eval("≢")];
    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_bqnvalue_slice() {
    let fns = &[eval("≠"), eval("≢")][..];
    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_iterator_f64() {
    let f = eval("+´");
    let ret = f.call1(&BQNValue::from_iter(
        [0.0f64, 1.0, 2.0, 3.0, 4.0].into_iter(),
    ));
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_iterator_i32() {
    let f = eval("+´");
    let ret = f.call1(&BQNValue::from_iter((0i32..).take(5)));
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_iterator_i16() {
    let f = eval("+´");
    let ret = f.call1(&(0i16..).take(5).collect::<BQNValue>());
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_iterator_i8() {
    let f = eval("+´");
    let ret = f.call1(&BQNValue::from_iter((0i8..).take(5)));
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_char() {
    let f = eval("+⟜1");
    let ret = f.call1(&'a'.into());
    assert_eq!(ret.to_char(), Some('b'));
}

#[test]
fn from_slice() {
    let f = eval("+´");
    let ret = f.call1(&[1i32, 2, 3, 4, 5].as_slice().into());
    assert_eq!(ret.to_f64(), 15.0);
}
