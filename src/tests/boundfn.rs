use super::*;

#[test]
fn fn1() -> Result<()> {
    let to_upper = BQNValue::fn1(|x| {
        let s = x.to_string().unwrap();
        BQNValue::from(&s.to_uppercase()[..])
    });
    assert_eq!(
        to_upper.call1(&"hello, world!".into())?.to_string()?,
        "HELLO, WORLD!"
    );

    Ok(())
}

#[test]
fn fn2() -> Result<()> {
    let split = BQNValue::fn2(|w, x| {
        x.to_string()
            .unwrap()
            .split(w.to_char().unwrap().unwrap())
            .collect::<Vec<_>>()
            .into()
    });
    assert_eq!(
        BQN!(split, "{' '𝕎𝕩}¨", ["Hello world!", "Rust ❤️ BQN"])?
            .to_bqnvalue_vec()?
            .iter()
            .map(|v| {
                v.to_bqnvalue_vec()
                    .unwrap()
                    .iter()
                    .map(|v| v.to_string().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<_>>>(),
        [vec!["Hello", "world!"], vec!["Rust", "❤️", "BQN"]]
    );
    Ok(())
}

#[test]
fn clone() -> Result<()> {
    let to_upper = BQNValue::fn1(|x| {
        let s = x.to_string().unwrap();
        BQNValue::from(&s.to_uppercase()[..])
    });
    let to_upper_2 = to_upper.clone();
    assert_eq!(
        to_upper.call1(&"hello, world!".into())?.to_string()?,
        "HELLO, WORLD!"
    );
    assert_eq!(
        to_upper_2.call1(&"hello, world!".into())?.to_string()?,
        "HELLO, WORLD!"
    );

    Ok(())
}

#[test]
#[ignore = "Ignored until I figure out how to make this work with Rust's changed extern C unwinding"]
#[should_panic]
fn boundfn_inside_boundfn() {
    let to_upper = BQNValue::fn1(|x| {
        let to_lower = BQNValue::fn1(|x| {
            let s = x.to_string().unwrap();
            BQNValue::from(&s.to_lowercase()[..])
        });
        let lower_x = to_lower.call1(x).unwrap();
        let s = lower_x.to_string().unwrap();
        BQNValue::from(&s.to_uppercase()[..])
    });
    let _ = to_upper.call1(&"hello".into());
}

#[test]
fn lifetime() -> Result<()> {
    fn boundfn() -> Result<BQNValue> {
        let f = BQNValue::fn1(|x| BQNValue::from(x.to_f64().unwrap() * 2.0));
        BQN!("⊢", f)
    }

    let f = boundfn()?;
    assert_eq!(f.call1(&1.0.into())?.to_f64()?, 2.0);

    Ok(())
}

#[test]
fn boundfn_count() {
    fn times2(x: &BQNValue) -> BQNValue {
        BQNValue::from(x.to_f64().unwrap() * 2.0)
    }

    let closure = |x: &BQNValue| BQNValue::from(x.to_f64().unwrap() * 2.0);

    // 1
    let _a = BQNValue::fn1(closure);
    let _b = BQNValue::fn1(closure);
    // 2
    let _c = BQNValue::fn1(|x| BQNValue::from(x.to_f64().unwrap() * 2.0));
    // 3
    let _d = BQNValue::fn1(|x| BQNValue::from(x.to_f64().unwrap() * 2.0));
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
