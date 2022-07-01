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
fn fn_inside_fn() {
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
