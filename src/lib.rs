use cbqn_sys::*;
use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use std::num::TryFromIntError;
use std::sync::Once;

mod macros;
use macros::*;

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

    pub fn has_field(&self, field: &str) -> bool {
        let _l = LOCK.lock();
        unsafe { bqn_hasField(self.value, BQNValue::from(field).value) }
    }

    pub fn get_field(&self, field: &str) -> Option<BQNValue> {
        let f = BQNValue::from(field);
        let _l = LOCK.lock();
        unsafe {
            if bqn_hasField(self.value, f.value) {
                Some(BQNValue::new(bqn_getField(self.value, f.value)))
            } else {
                None
            }
        }
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

        let b = self.bound()?;
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

        let b = self.bound()?;
        let mut ret = Vec::with_capacity(b);
        unsafe {
            bqn_readI32Arr(self.value, ret.as_mut_ptr());
            drop(l);
            ret.set_len(b);
        }

        Ok(ret)
    }

    pub fn into_string(self) -> Result<String, TryFromIntError> {
        let l = LOCK.lock();

        let b = self.bound()?;
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

    fn bound(&self) -> Result<usize, TryFromIntError> {
        unsafe { bqn_bound(self.value) }.try_into()
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

impl_from_array!(f64, bqn_makeF64Vec);
impl_from_array!(i32, bqn_makeI32Vec);
impl_from_array!(i16, bqn_makeI16Vec);
impl_from_array!(i8, bqn_makeI8Vec);

impl_from_slice!(&[f64], bqn_makeF64Vec);
impl_from_slice!(&[i32], bqn_makeI32Vec);
impl_from_slice!(&[i16], bqn_makeI16Vec);
impl_from_slice!(&[i8], bqn_makeI8Vec);

impl_from_iterator!(f64, bqn_makeF64Vec);
impl_from_iterator!(i32, bqn_makeI32Vec);
impl_from_iterator!(i16, bqn_makeI16Vec);
impl_from_iterator!(i8, bqn_makeI8Vec);

/// Evaluates BQN code
pub fn eval(bqn: &str) -> BQNValue {
    INIT.call_once(|| {
        let _l = LOCK.lock();
        unsafe { bqn_init() }
    });
    let _l = LOCK.lock();
    let ret = BQNValue::new(unsafe { bqn_eval(BQNValue::from(bqn).value) });
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
    fn fixed_size_array() {
        let f = eval("+´");
        f.call1(&[0.0, 1.0, 2.0, 3.0, 4.0].into());
    }

    #[test]
    fn from_iterator_f64() {
        let f = eval("+´");
        let ret = f.call1(&BQNValue::from_iter(
            [0.0f64, 1.0, 2.0, 3.0, 4.0].into_iter(),
        ));
        assert_eq!(ret.into_f64(), 10.0);
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
        let ret = f.call1(&(0i16..).take(5).collect::<BQNValue>());
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

    #[test]
    fn bqn_macro() {
        assert_eq!(BQN!("+´", [1, 2, 3]).into_f64(), 6.0);
        assert_eq!(BQN!('a', "+", 1).into_char(), Some('b'));
        let arr = BQN!("+`", [1, 2, 3]);
        assert_eq!(BQN!(2, "×", arr).into_i32_vec(), Ok(vec![2, 6, 12]));
    }
}
