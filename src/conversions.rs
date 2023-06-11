use crate::{backend::*, macros::*, BQNValue, LOCK};

impl From<f64> for BQNValue {
    fn from(v: f64) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(bqn_makeF64(v).unwrap())
    }
}

impl From<i32> for BQNValue {
    fn from(v: i32) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(bqn_makeF64(v as f64).unwrap())
    }
}

impl From<char> for BQNValue {
    fn from(v: char) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(bqn_makeChar(v as u32).unwrap())
    }
}

impl_from_string_like!(&str);
impl_from_string_like!(&String);
impl_from_string_like!(String);
impl_from_string_like_vec!(&str);
impl_from_string_like_vec!(&String);
impl_from_string_like_vec!(String);

impl<const N: usize> From<[&str; N]> for BQNValue {
    fn from(arr: [&str; N]) -> BQNValue {
        crate::INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap()
        });
        let mut strs = Vec::with_capacity(N);
        let _l = LOCK.lock();
        for s in arr {
            strs.push(bqn_makeUTF8Str(s).unwrap());
        }
        BQNValue::new(bqn_makeObjVec(&strs).unwrap())
    }
}

impl<const N: usize> From<[String; N]> for BQNValue {
    fn from(arr: [String; N]) -> BQNValue {
        crate::INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap();
        });
        let mut strs = Vec::with_capacity(N);
        let _l = LOCK.lock();
        for s in arr {
            strs.push(bqn_makeUTF8Str(&s).unwrap());
        }
        BQNValue::new(bqn_makeObjVec(&strs).unwrap())
    }
}

impl_from_array!(f64, bqn_makeF64Vec);
impl_from_array!(i32, bqn_makeI32Vec);
impl_from_array!(i16, bqn_makeI16Vec);
impl_from_array!(i8, bqn_makeI8Vec);
impl<const N: usize> From<[BQNValue; N]> for BQNValue {
    fn from(arr: [BQNValue; N]) -> BQNValue {
        crate::INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap()
        });
        let elems = arr.into_iter().map(|v| v.value).collect::<Vec<_>>();
        let _l = LOCK.lock();
        BQNValue::new(bqn_makeObjVec(&elems).unwrap())
    }
}

impl_from_slice!(&[f64], bqn_makeF64Vec);
impl_from_slice!(&[i32], bqn_makeI32Vec);
impl_from_slice!(&[i16], bqn_makeI16Vec);
impl_from_slice!(&[i8], bqn_makeI8Vec);
impl From<&[BQNValue]> for BQNValue {
    fn from(arr: &[BQNValue]) -> BQNValue {
        crate::INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap()
        });

        let elems = arr.iter().map(|v| v.value).collect::<Vec<_>>();
        let _l = LOCK.lock();
        BQNValue::new(bqn_makeObjVec(&elems).unwrap())
    }
}

impl_from_vec!(f64, bqn_makeF64Vec);
impl_from_vec!(i32, bqn_makeI32Vec);
impl_from_vec!(i16, bqn_makeI16Vec);
impl_from_vec!(i8, bqn_makeI8Vec);
impl From<Vec<BQNValue>> for BQNValue {
    fn from(arr: Vec<BQNValue>) -> BQNValue {
        crate::INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap();
        });

        let elems = arr.into_iter().map(|v| v.value).collect::<Vec<_>>();
        let _l = LOCK.lock();
        BQNValue::new(bqn_makeObjVec(&elems).unwrap())
    }
}

impl_from_iterator!(f64, bqn_makeF64Vec);
impl_from_iterator!(i32, bqn_makeI32Vec);
impl_from_iterator!(i16, bqn_makeI16Vec);
impl_from_iterator!(i8, bqn_makeI8Vec);
impl FromIterator<BQNValue> for BQNValue {
    fn from_iter<T>(iter: T) -> BQNValue
    where
        T: IntoIterator<Item = BQNValue>,
    {
        crate::INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap();
        });

        let elems = iter.into_iter().map(|v| v.value).collect::<Vec<_>>();
        let _l = LOCK.lock();
        BQNValue::new(bqn_makeObjVec(&elems).unwrap())
    }
}
