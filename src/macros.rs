macro_rules! impl_from_slice {
    ($ty:ty, $fn:ident) => {
        impl From<$ty> for BQNValue {
            fn from(arr: $ty) -> BQNValue {
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
                let elems = iter.into_iter().collect::<Vec<_>>();
                let len = elems.len();
                let _l = LOCK.lock();
                BQNValue::new(unsafe { $fn(len.try_into().unwrap(), elems.as_ptr()) })
            }
        }
    };
}

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
