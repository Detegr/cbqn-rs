macro_rules! impl_from_string_like {
    ($ty:ty) => {
        impl From<$ty> for BQNValue {
            fn from(v: $ty) -> BQNValue {
                crate::INIT.call_once(|| {
                    let _l = LOCK.lock();
                    bqn_init().unwrap();
                });
                let _l = LOCK.lock();
                BQNValue::new(bqn_makeUTF8Str(&v).unwrap())
            }
        }
    };
}

macro_rules! impl_from_string_like_vec {
    ($ty:ty) => {
        impl From<Vec<$ty>> for BQNValue {
            fn from(arr: Vec<$ty>) -> BQNValue {
                crate::INIT.call_once(|| {
                    let _l = LOCK.lock();
                    bqn_init().unwrap();
                });
                let mut strs = Vec::with_capacity(arr.len());
                let _l = LOCK.lock();
                for s in &arr {
                    strs.push(bqn_makeUTF8Str(&s).unwrap());
                }
                BQNValue::new(bqn_makeObjVec(&strs).unwrap())
            }
        }
    };
}

macro_rules! impl_from_slice {
    ($ty:ty, $fn:ident) => {
        impl From<$ty> for BQNValue {
            fn from(arr: $ty) -> BQNValue {
                crate::INIT.call_once(|| {
                    let _l = LOCK.lock();
                    bqn_init().unwrap();
                });

                let _l = LOCK.lock();
                BQNValue::new($fn(&arr).unwrap())
            }
        }
    };
}

macro_rules! impl_from_array {
    ($ty:ty, $fn:ident) => {
        impl<const N: usize> From<[$ty; N]> for BQNValue {
            fn from(arr: [$ty; N]) -> BQNValue {
                crate::INIT.call_once(|| {
                    let _l = LOCK.lock();
                    bqn_init().unwrap();
                });

                let _l = LOCK.lock();
                BQNValue::new($fn(&arr).unwrap())
            }
        }
    };
}

macro_rules! impl_from_iterator {
    ($ty:ty, $fn:ident) => {
        impl FromIterator<$ty> for BQNValue {
            fn from_iter<T>(iter: T) -> BQNValue
            where
                T: IntoIterator<Item = $ty>,
            {
                crate::INIT.call_once(|| {
                    let _l = LOCK.lock();
                    bqn_init().unwrap();
                });

                let elems = iter.into_iter().collect::<Vec<_>>();
                let _l = LOCK.lock();
                BQNValue::new($fn(&elems).unwrap())
            }
        }
    };
}

macro_rules! impl_from_vec {
    ($ty:ty, $fn:ident) => {
        impl From<Vec<$ty>> for BQNValue {
            fn from(arr: Vec<$ty>) -> BQNValue {
                crate::INIT.call_once(|| {
                    let _l = LOCK.lock();
                    bqn_init().unwrap();
                });

                let _l = LOCK.lock();
                BQNValue::new($fn(&arr).unwrap())
            }
        }
    };
}

/// Convenience macro for running BQN expressions
///
/// Takes a string of BQN code and optional left and right argument
/// # Examples
/// ```
/// # use cbqn::{BQN, BQNValue, eval};
/// let sum = BQN!("1+1").unwrap();
/// assert_eq!(sum.to_f64().unwrap(), 2.0);
/// ```
///
/// ```
/// # use cbqn::{BQN, BQNValue, eval};
/// let bqn_is_anagram = BQN!("⌽≡⊢", "BQN").and_then(|e| e.to_f64()).unwrap();
/// assert_eq!(bqn_is_anagram, 0.0);
/// ```
///
/// ```
/// # use cbqn::{BQN, BQNValue, eval};
/// let strs = BQN!(' ', "(⊢-˜+`×¬)∘=⊔⊢", "Rust ❤️ BQN")
///     .and_then(|e| e.to_bqnvalue_vec())
///     .and_then(|v| {
///         v.iter()
///             .map(BQNValue::to_string)
///             .collect::<Result<Vec<String>, _>>()
///     })
///     .unwrap();
/// assert_eq!(strs, ["Rust", "❤️", "BQN"]);
/// ```
///
/// ```
/// # use cbqn::{BQN, BQNValue, eval};
/// let strings = ["join", "these", "please"];
/// assert_eq!(BQN!("∾", strings).and_then(|e| e.to_string()).unwrap(), "jointheseplease");
/// ```
#[macro_export]
macro_rules! BQN {
    ($code:expr) => {
        eval($code)
    };
    ($code:expr, $x:expr) => {
        eval($code).and_then(|v| v.call1(&BQNValue::from($x)))
    };
    ($w:expr, $code:expr, $x:expr) => {
        eval($code).and_then(|v| v.call2(&BQNValue::from($w), &BQNValue::from($x)))
    };
}

pub(crate) use impl_from_array;
pub(crate) use impl_from_iterator;
pub(crate) use impl_from_slice;
pub(crate) use impl_from_string_like;
pub(crate) use impl_from_string_like_vec;
pub(crate) use impl_from_vec;
