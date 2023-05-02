use super::*;

#[test]
fn to_char() -> Result<()> {
    let ret = eval(r#"⊑"hello""#)?;
    assert_eq!(ret.to_char()?, Some('h'));
    Ok(())
}

#[test]
fn to_u32() -> Result<()> {
    let ret = eval(r#"⊑"hello""#)?;
    assert_eq!(ret.to_u32()?, 104);
    Ok(())
}

#[test]
fn to_f64_vec() -> Result<()> {
    let ret = eval("2‿∘⥊↕6")?;
    assert_eq!(ret.to_f64_vec()?, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    Ok(())
}

#[test]
fn elt_unk_to_f64_vec() -> Result<()> {
    let ret = eval(r#"1↓"abc"∾2‿∘⥊↕6"#)?;
    assert_eq!(ret.to_f64_vec()?, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    Ok(())
}

#[test]
fn to_bqnvalue_vec() -> Result<()> {
    let strings = BQN!("↑", "hello")?
        .to_bqnvalue_vec()?
        .into_iter()
        .map(|v| v.to_string().unwrap())
        .collect::<Vec<String>>();
    assert_eq!(strings, vec!["", "h", "he", "hel", "hell", "hello"]);
    Ok(())
}

#[test]
fn should_panic_number_to_bqnvalue_vec() -> Result<()> {
    let v = BQN!("123")?;
    assert!(panic::catch_unwind(move || {
        v.to_bqnvalue_vec().unwrap();
    })
    .is_err());
    Ok(())
}

#[test]
fn elt_unk_to_string() -> Result<()> {
    let v = BQN!(r#"1↓0∾"aaa""#)?;
    assert_eq!(v.to_string()?, "aaa");
    Ok(())
}
