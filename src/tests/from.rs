use super::*;

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
