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

#[test]
fn call1() -> Result<()> {
    let f = eval("↕")?;
    let ret = f.call1(&5.into())?;
    assert_eq!(ret.to_f64_vec()?, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    Ok(())
}

#[test]
fn call2() -> Result<()> {
    let f = eval("⊑")?;
    let ret = f.call2(&3.into(), &"hello".into())?;
    assert_eq!(ret.to_char()?, Some('l'));
    Ok(())
}

#[test]
fn fixed_size_array() -> Result<()> {
    let f = eval("+´")?;
    let ret = f.call1(&[0.0, 1.0, 2.0, 3.0, 4.0].into())?;
    assert_eq!(ret.to_f64()?, 10.0);
    Ok(())
}

#[test]
fn bqn_macro() -> Result<()> {
    assert_eq!(BQN!("3")?.to_f64()?, 3.0);
    assert_eq!(BQN!("+´", [1, 2, 3])?.to_f64()?, 6.0);
    assert_eq!(BQN!('a', "+", 1)?.to_char()?, Some('b'));
    let arr = BQN!("+`", [1, 2, 3])?;
    assert_eq!(BQN!(2, "×", arr)?.to_f64_vec()?, vec![2.0, 6.0, 12.0]);
    Ok(())
}

#[test]
fn test_debug_repr() -> Result<()> {
    let v = BQN!("1‿2‿3")?;
    assert_eq!(format!("{:?}", v), "⟨ 1 2 3 ⟩");
    Ok(())
}

#[test]
fn namespace() -> Result<()> {
    let ns = ns();

    assert!(ns.has_field("a")?);
    assert!(ns.get_field("a")?.is_some());
    assert!(!ns.has_field("A")?);
    assert!(ns.get_field("A")?.is_none());
    assert!(ns.has_field("b")?);
    assert!(ns.get_field("b")?.is_some());
    assert!(!ns.has_field("B")?);
    assert!(ns.get_field("B")?.is_none());
    assert!(!ns.has_field("c")?);
    assert!(ns.get_field("c")?.is_none());

    assert_eq!(
        ns.get_field("a")?.as_ref().map(|f| f.to_f64().unwrap()),
        Some(1.0)
    );
    assert_eq!(
        ns.get_field("b")?
            .map(|b| b.call1(&1.into()).unwrap().to_f64().unwrap()),
        Some(2.0)
    );
    Ok(())
}

#[test]
fn clone() -> Result<()> {
    let v = BQNValue::from("hello");
    assert_eq!(BQN!(v.clone(), "≡", v)?.to_f64()?, 1.0);
    Ok(())
}

#[test]
fn rank() -> Result<()> {
    let rank0 = BQN!("<0")?;
    let rank1 = BQN!("↕5")?;
    let rank2 = BQN!("2‿2⥊5")?;
    let rank3 = BQN!("2‿2‿2⥊5")?;

    assert_eq!(rank0.rank(), 0);
    assert_eq!(rank1.rank(), 1);
    assert_eq!(rank2.rank(), 2);
    assert_eq!(rank3.rank(), 3);

    Ok(())
}

#[test]
fn shape() -> Result<()> {
    let shape0 = BQN!("<5")?;
    let shape1 = BQN!("↕5")?;
    let shape2 = BQN!("2‿2⥊5")?;
    let shape3 = BQN!("2‿2‿2⥊5")?;

    assert_eq!(shape0.shape(), vec![] as Vec<usize>);
    assert_eq!(shape1.shape(), vec![5]);
    assert_eq!(shape2.shape(), vec![2, 2]);
    assert_eq!(shape3.shape(), vec![2, 2, 2]);

    Ok(())
}
