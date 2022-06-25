use cbqn_sys::*;
use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use std::num::TryFromIntError;
use std::sync::Once;

static LOCK: Lazy<ReentrantMutex<()>> = Lazy::new(|| ReentrantMutex::new(()));

static INIT: Once = Once::new();
pub struct BQNValue {
    value: BQNV,
}

impl BQNValue {
    fn new(value: BQNV) -> BQNValue {
        BQNValue { value }
    }

    pub fn null() -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeChar(0) })
    }

    /// Calls BQN function with 1 argument
    pub fn call1(&self, x: &BQNValue) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_call1(self.value, x.value) })
    }

    /// Calls BQN function with 2 arguments
    pub fn call2(&self, w: &BQNValue, x: &BQNValue) -> BQNValue {
        let _l = LOCK.lock();
        unsafe { BQNValue::new(bqn_call2(self.value, w.value, x.value)) }
    }

    pub fn into_f64(self) -> f64 {
        let _l = LOCK.lock();
        unsafe { bqn_toF64(self.value) }
    }

    pub fn into_char(self) -> Option<char> {
        let _l = LOCK.lock();
        unsafe { char::from_u32(bqn_toChar(self.value)) }
    }

    pub fn into_u32(self) -> u32 {
        let _l = LOCK.lock();
        unsafe { bqn_toChar(self.value) }
    }

    pub fn into_f64_vec(self) -> Result<Vec<f64>, TryFromIntError> {
        let l = LOCK.lock();

        let b = unsafe { bqn_bound(self.value) }.try_into()?;
        let mut ret = Vec::with_capacity(b);
        unsafe {
            bqn_readF64Arr(self.value, ret.as_mut_ptr());
            drop(l);
            ret.set_len(b);
        }

        Ok(ret)
    }

    pub fn into_i32_vec(self) -> Result<Vec<i32>, TryFromIntError> {
        let l = LOCK.lock();

        let b = unsafe { bqn_bound(self.value) }.try_into()?;
        let mut ret = Vec::with_capacity(b);
        unsafe {
            bqn_readI32Arr(self.value, ret.as_mut_ptr());
            drop(l);
            ret.set_len(b);
        }

        Ok(ret)
    }

    pub fn into_char_vec(self) -> Result<Vec<char>, TryFromIntError> {
        let l = LOCK.lock();

        let b = unsafe { bqn_bound(self.value) }.try_into()?;
        let mut u32s = Vec::with_capacity(b);
        unsafe {
            bqn_readC32Arr(self.value, u32s.as_mut_ptr());
            drop(l);
            u32s.set_len(b);
        }

        Ok(u32s
            .into_iter()
            .map(|c| unsafe { char::from_u32_unchecked(c) })
            .collect())
    }

    pub fn bound(&self) -> u32 {
        unsafe { bqn_bound(self.value) }.try_into().unwrap()
    }
}

impl Drop for BQNValue {
    fn drop(&mut self) {
        let _l = LOCK.lock();
        unsafe { bqn_free(self.value) };
    }
}

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

impl From<&str> for BQNValue {
    fn from(v: &str) -> BQNValue {
        let str_bytes = v.as_bytes();
        let _l = LOCK.lock();
        BQNValue::new(unsafe {
            bqn_makeUTF8Str(
                str_bytes.len().try_into().unwrap(),
                str_bytes.as_ptr() as *const i8,
            )
        })
    }
}

impl From<&[f64]> for BQNValue {
    fn from(arr: &[f64]) -> BQNValue {
        let len = arr.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeF64Vec(len.try_into().unwrap(), arr.as_ptr()) })
    }
}

impl From<&[i32]> for BQNValue {
    fn from(arr: &[i32]) -> BQNValue {
        let len = arr.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeI32Vec(len.try_into().unwrap(), arr.as_ptr()) })
    }
}

impl From<&[i16]> for BQNValue {
    fn from(arr: &[i16]) -> BQNValue {
        let len = arr.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeI16Vec(len.try_into().unwrap(), arr.as_ptr()) })
    }
}

impl From<&[i8]> for BQNValue {
    fn from(arr: &[i8]) -> BQNValue {
        let len = arr.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeI8Vec(len.try_into().unwrap(), arr.as_ptr()) })
    }
}

impl FromIterator<f64> for BQNValue {
    fn from_iter<T>(iter: T) -> BQNValue
    where
        T: IntoIterator<Item = f64>,
    {
        let elems = iter.into_iter().collect::<Vec<_>>();
        let len = elems.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeF64Vec(len.try_into().unwrap(), elems.as_ptr()) })
    }
}

impl FromIterator<i32> for BQNValue {
    fn from_iter<T>(iter: T) -> BQNValue
    where
        T: IntoIterator<Item = i32>,
    {
        let elems = iter.into_iter().collect::<Vec<_>>();
        let len = elems.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeI32Vec(len.try_into().unwrap(), elems.as_ptr()) })
    }
}

impl FromIterator<i16> for BQNValue {
    fn from_iter<T>(iter: T) -> BQNValue
    where
        T: IntoIterator<Item = i16>,
    {
        let elems = iter.into_iter().collect::<Vec<_>>();
        let len = elems.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeI16Vec(len.try_into().unwrap(), elems.as_ptr()) })
    }
}

impl FromIterator<i8> for BQNValue {
    fn from_iter<T>(iter: T) -> BQNValue
    where
        T: IntoIterator<Item = i8>,
    {
        let elems = iter.into_iter().collect::<Vec<_>>();
        let len = elems.len();
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_makeI8Vec(len.try_into().unwrap(), elems.as_ptr()) })
    }
}

/// Evaluates BQN code
pub fn eval(bqn: &str) -> BQNValue {
    INIT.call_once(|| {
        let _l = LOCK.lock();
        unsafe { bqn_init() }
    });
    let str_bytes = bqn.as_bytes();
    let _l = LOCK.lock();
    let bqn_str = unsafe {
        bqn_makeUTF8Str(
            str_bytes.len().try_into().unwrap(),
            str_bytes.as_ptr() as *const i8,
        )
    };
    let ret = BQNValue::new(unsafe { bqn_eval(bqn_str) });
    unsafe { bqn_free(bqn_str) }
    ret
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn into_char() {
        let ret = eval(r#"⊑"hello""#);
        assert_eq!(ret.into_char(), Some('h'));
    }
    #[test]
    fn into_u32() {
        let ret = eval(r#"⊑"hello""#);
        assert_eq!(ret.into_u32(), 104);
    }

    #[test]
    fn into_f64_vec() {
        let ret = eval("2‿∘⥊↕6");
        assert_eq!(ret.into_f64_vec(), Ok(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]));
    }

    #[test]
    fn into_u32_vec() {
        let ret = eval("0.25+↕5");
        assert_eq!(ret.into_i32_vec(), Ok(vec![0, 1, 2, 3, 4]));
    }
    #[test]
    fn call1() {
        let f = eval("↕");
        let ret = f.call1(&5.into());
        assert_eq!(ret.into_i32_vec(), Ok(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn call2() {
        let f = eval("⊑");
        let ret = f.call2(&3.into(), &"hello".into());
        assert_eq!(ret.into_char(), Some('l'));
    }

    #[test]
    fn from_iterator_i32() {
        let f = eval("+´");
        let ret = f.call1(&BQNValue::from_iter((0i32..).take(5)));
        assert_eq!(ret.into_f64(), 10.0);
    }

    #[test]
    fn from_iterator_i16() {
        let f = eval("+´");
        let ret = f.call1(&BQNValue::from_iter((0i16..).take(5)));
        assert_eq!(ret.into_f64(), 10.0);
    }

    #[test]
    fn from_iterator_i8() {
        let f = eval("+´");
        let ret = f.call1(&BQNValue::from_iter((0i8..).take(5)));
        assert_eq!(ret.into_f64(), 10.0);
    }

    #[test]
    fn from_char() {
        let f = eval("+⟜1");
        let ret = f.call1(&'a'.into());
        assert_eq!(ret.into_char(), Some('b'));
    }

    #[test]
    fn from_slice() {
        let f = eval("+´");
        let ret = f.call1(&[1i32, 2, 3, 4, 5].as_slice().into());
        assert_eq!(ret.into_f64(), 15.0);
    }
}
