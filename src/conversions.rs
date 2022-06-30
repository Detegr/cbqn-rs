use crate::{macros::*, BQNValue, LOCK};
use cbqn_sys::*;

impl From<f64> for BQNValue {
    fn from(v: f64) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeF64(v) })
    }
}

impl From<i32> for BQNValue {
    fn from(v: i32) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeF64(v as f64) })
    }
}

impl From<char> for BQNValue {
    fn from(v: char) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeChar(v as u32) })
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
        let mut strs = Vec::with_capacity(N);
        for s in arr {
            let str_bytes = s.as_bytes();
            strs.push(unsafe {
                bqn_makeUTF8Str(
                    str_bytes.len().try_into().unwrap(),
                    str_bytes.as_ptr() as *const i8,
                )
            });
        }
        BQNValue::new(unsafe { bqn_makeObjVec(N as u64, strs.as_ptr()) })
    }
}

impl<const N: usize> From<[String; N]> for BQNValue {
    fn from(arr: [String; N]) -> BQNValue {
        let mut strs = Vec::with_capacity(N);
        for s in arr {
            let str_bytes = s.as_bytes();
            strs.push(unsafe {
                bqn_makeUTF8Str(
                    str_bytes.len().try_into().unwrap(),
                    str_bytes.as_ptr() as *const i8,
                )
            });
        }
        BQNValue::new(unsafe { bqn_makeObjVec(N as u64, strs.as_ptr()) })
    }
}

impl_from_array!(f64, bqn_makeF64Vec);
impl_from_array!(i32, bqn_makeI32Vec);
impl_from_array!(i16, bqn_makeI16Vec);
impl_from_array!(i8, bqn_makeI8Vec);

impl_from_slice!(&[f64], bqn_makeF64Vec);
impl_from_slice!(&[i32], bqn_makeI32Vec);
impl_from_slice!(&[i16], bqn_makeI16Vec);
impl_from_slice!(&[i8], bqn_makeI8Vec);

impl_from_vec!(f64, bqn_makeF64Vec);
impl_from_vec!(i32, bqn_makeI32Vec);
impl_from_vec!(i16, bqn_makeI16Vec);
impl_from_vec!(i8, bqn_makeI8Vec);

impl_from_iterator!(f64, bqn_makeF64Vec);
impl_from_iterator!(i32, bqn_makeI32Vec);
impl_from_iterator!(i16, bqn_makeI16Vec);
impl_from_iterator!(i8, bqn_makeI8Vec);
