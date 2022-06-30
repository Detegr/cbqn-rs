macro_rules! impl_from_slice {
    ($ty:ty, $fn:ident) => {
        impl From<$ty> for BQNValue {
            fn from(arr: $ty) -> BQNValue {
                crate::INIT.call_once(|| {
                    let _l = LOCK.lock();
                    unsafe { bqn_init() }
                });

                let len = arr.len();
                let _l = LOCK.lock();
                BQNValue::new(unsafe { $fn(len.try_into().unwrap(), arr.as_ptr()) })
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
                    unsafe { bqn_init() }
                });

                let len = arr.len();
                let _l = LOCK.lock();
                BQNValue::new(unsafe { $fn(len.try_into().unwrap(), arr.as_ptr()) })
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
                    unsafe { bqn_init() }
                });

                let elems = iter.into_iter().collect::<Vec<_>>();
                let len = elems.len();
                let _l = LOCK.lock();
                BQNValue::new(unsafe { $fn(len.try_into().unwrap(), elems.as_ptr()) })
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
                    unsafe { bqn_init() }
                });

                let len = arr.len();
                let _l = LOCK.lock();
                BQNValue::new(unsafe { $fn(len.try_into().unwrap(), arr.as_ptr()) })
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
/// let sum = BQN!("1+1");
/// assert_eq!(sum.to_f64(), 2.0);
/// ```
///
/// ```
/// # use cbqn::{BQN, BQNValue, eval};
/// let bqn_is_anagram = BQN!("⌽≡⊢", "BQN");
/// assert_eq!(bqn_is_anagram.to_f64(), 0.0);
/// ```
///
/// ```
/// # use cbqn::{BQN, BQNValue, eval};
/// let strs = BQN!(' ', "(⊢-˜+`×¬)∘=⊔⊢", "Rust ❤️ BQN")
///     .to_bqnvalue_vec()
///     .iter()
///     .map(BQNValue::to_string)
///     .collect::<Vec<String>>();
/// assert_eq!(strs, ["Rust", "❤️", "BQN"]);
/// ```
///
/// ```
/// # use cbqn::{BQN, BQNValue, eval};
/// let strings = ["join", "these", "please"];
/// assert_eq!(BQN!("∾", strings).to_string(), "jointheseplease");
/// ```
#[macro_export]
macro_rules! BQN {
    ($code:expr) => {
        eval($code)
    };
    ($code:expr, $x:expr) => {
        eval($code).call1(&BQNValue::from($x))
    };
    ($w:expr, $code:expr, $x:expr) => {
        eval($code).call2(&BQNValue::from($w), &BQNValue::from($x))
    };
}

pub(crate) use impl_from_array;
pub(crate) use impl_from_iterator;
pub(crate) use impl_from_slice;
pub(crate) use impl_from_vec;
