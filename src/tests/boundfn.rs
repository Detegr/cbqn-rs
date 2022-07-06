use super::*;

#[test]
fn fn1() {
    let to_upper = BQNValue::fn1(|x| {
        let s = x.to_string();
        BQNValue::from(&s.to_uppercase()[..])
    });
    assert_eq!(
        to_upper.call1(&"hello, world!".into()).to_string(),
        "HELLO, WORLD!"
    );
}

#[test]
fn fn2() {
    let split = BQNValue::fn2(|w, x| {
        x.to_string()
            .split(w.to_char().unwrap())
            .collect::<Vec<_>>()
            .into()
    });
    assert_eq!(
        BQN!(split, "{' '𝕎𝕩}¨", ["Hello world!", "Rust ❤️ BQN"])
            .to_bqnvalue_vec()
            .iter()
            .map(|v| {
                v.to_bqnvalue_vec()
                    .iter()
                    .map(BQNValue::to_string)
                    .collect()
            })
            .collect::<Vec<Vec<_>>>(),
        [vec!["Hello", "world!"], vec!["Rust", "❤️", "BQN"]]
    );
}

#[test]
fn clone() {
    let to_upper = BQNValue::fn1(|x| {
        let s = x.to_string();
        BQNValue::from(&s.to_uppercase()[..])
    });
    let to_upper_2 = to_upper.clone();
    assert_eq!(
        to_upper.call1(&"hello, world!".into()).to_string(),
        "HELLO, WORLD!"
    );
    assert_eq!(
        to_upper_2.call1(&"hello, world!".into()).to_string(),
        "HELLO, WORLD!"
    );
}

#[test]
#[should_panic]
fn boundfn_inside_boundfn() {
    let to_upper = BQNValue::fn1(|x| {
        let to_lower = BQNValue::fn1(|x| {
            let s = x.to_string();
            BQNValue::from(&s.to_lowercase()[..])
        });
        let lower_x = to_lower.call1(x);
        let s = lower_x.to_string();
        BQNValue::from(&s.to_uppercase()[..])
    });
    assert_eq!(
        to_upper.call1(&"hello, world!".into()).to_string(),
        "HELLO, WORLD!"
    );
}

#[test]
fn lifetime() {
    fn boundfn() -> BQNValue {
        let f = BQNValue::fn1(|x| BQNValue::from(x.to_f64() * 2.0));
        BQN!("⊢", f)
    }

    let f = boundfn();
    assert_eq!(f.call1(&1.0.into()).to_f64(), 2.0);
}

#[test]
fn boundfn_count() {
    fn times2(x: &BQNValue) -> BQNValue {
        BQNValue::from(x.to_f64() * 2.0)
    }

    let closure = |x: &BQNValue| BQNValue::from(x.to_f64() * 2.0);

    // 1
    let _a = BQNValue::fn1(closure);
    let _b = BQNValue::fn1(closure);
    // 2
    let _c = BQNValue::fn1(|x| BQNValue::from(x.to_f64() * 2.0));
    // 3
    let _d = BQNValue::fn1(|x| BQNValue::from(x.to_f64() * 2.0));
    // 4
    let _e = BQNValue::fn1(times2);
    let _f = BQNValue::fn1(times2);

    // 5
    let _v = (0..5)
        .map(|_| BQNValue::fn1(|x| x.clone()))
        .collect::<Vec<BQNValue>>();

    FNS.with(|fns| {
        let fns = fns.borrow();
        assert_eq!(fns.boundfn_1.len(), 5);
    });
}
