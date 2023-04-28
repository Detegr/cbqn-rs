use super::*;

#[test]
fn from_iterator_bqnvalue() {
    let fns = "≠≢"
        .chars()
        .map(|c| eval(&c.to_string()).unwrap())
        .collect::<BQNValue>();

    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).unwrap().to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_bqnvalue_vec() {
    let fns = vec![eval("≠").unwrap(), eval("≢").unwrap()];
    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).unwrap().to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_bqnvalue_array() {
    let fns = [eval("≠").unwrap(), eval("≢").unwrap()];
    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).unwrap().to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_bqnvalue_slice() {
    let fns = &[eval("≠").unwrap(), eval("≢").unwrap()][..];
    let ret = &BQN!("{𝕏3‿3⥊0}¨", fns).unwrap().to_bqnvalue_vec()[..];
    assert_eq!(ret[0].to_f64(), 3.0);
    let shape = ret[1].to_bqnvalue_vec();
    assert_eq!(shape.len(), 2);
    for v in shape {
        assert_eq!(v.to_f64(), 3.0);
    }
}

#[test]
fn from_iterator_f64() {
    let f = eval("+´").unwrap();
    let ret = f
        .call1(&BQNValue::from_iter(
            [0.0f64, 1.0, 2.0, 3.0, 4.0].into_iter(),
        ))
        .unwrap();
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_iterator_i32() {
    let f = eval("+´").unwrap();
    let ret = f.call1(&BQNValue::from_iter((0i32..).take(5))).unwrap();
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_iterator_i16() {
    let f = eval("+´").unwrap();
    let ret = f.call1(&(0i16..).take(5).collect::<BQNValue>()).unwrap();
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_iterator_i8() {
    let f = eval("+´").unwrap();
    let ret = f.call1(&BQNValue::from_iter((0i8..).take(5))).unwrap();
    assert_eq!(ret.to_f64(), 10.0);
}

#[test]
fn from_char() {
    let f = eval("+⟜1").unwrap();
    let ret = f.call1(&'a'.into()).unwrap();
    assert_eq!(ret.to_char(), Some('b'));
}

#[test]
fn from_slice() {
    let f = eval("+´").unwrap();
    let ret = f.call1(&[1i32, 2, 3, 4, 5].as_slice().into()).unwrap();
    assert_eq!(ret.to_f64(), 15.0);
}
