use cbqn_sys::*;
use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use std::{fmt, sync::Once};

mod macros;
use macros::*;

static LOCK: Lazy<ReentrantMutex<()>> = Lazy::new(|| ReentrantMutex::new(()));

static INIT: Once = Once::new();

// Fields of this struct must not be altered.
// It has to have the same in-memory representation than plain BQNV
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
        if self.bqn_type() != BQNType::Namespace {
            panic!("value isn't a namespace");
        }
        unsafe { bqn_hasField(self.value, BQNValue::from(field).value) }
    }

    pub fn get_field(&self, field: &str) -> Option<BQNValue> {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Namespace {
            panic!("value isn't a namespace");
        }
        let f = BQNValue::from(field);
        if !bqneltype_is_char(f.direct_arr_type()) {
            panic!("field is not a string");
        }
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
        if self.bqn_type() != BQNType::Number {
            panic!("value isn't a number");
        }
        unsafe { bqn_toF64(self.value) }
    }

    pub fn into_char(self) -> Option<char> {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            panic!("value isn't a character");
        }
        unsafe { char::from_u32(bqn_toChar(self.value)) }
    }

    pub fn into_u32(self) -> u32 {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            panic!("value isn't a character");
        }
        unsafe { bqn_toChar(self.value) }
    }

    pub fn into_f64_vec(self) -> Vec<f64> {
        let l = LOCK.lock();
        if !bqneltype_is_numeric(self.direct_arr_type()) {
            panic!("value isn't a f64 array");
        }

        let b = self.bound();
        let mut ret = Vec::with_capacity(b);
        unsafe {
            bqn_readF64Arr(self.value, ret.as_mut_ptr());
            drop(l);
            ret.set_len(b);
        }

        ret
    }

    pub fn into_i32_vec(self) -> Vec<i32> {
        let l = LOCK.lock();
        if !bqneltype_is_numeric(self.direct_arr_type()) {
            panic!("value isn't an i32 array");
        }

        let b = self.bound();
        let mut ret = Vec::with_capacity(b);
        unsafe {
            bqn_readI32Arr(self.value, ret.as_mut_ptr());
            drop(l);
            ret.set_len(b);
        }

        ret
    }

    pub fn into_bqnvalue_vec(self) -> Vec<BQNValue> {
        let l = LOCK.lock();
        if !bqneltype_is_unknown(self.direct_arr_type()) {
            panic!("value isn't an object array");
        } else {
            if self.bqn_type() != BQNType::Array {
                panic!("value isn't an object array");
            }
        }

        let b = self.bound();
        let mut ret = Vec::with_capacity(b);
        unsafe {
            bqn_readObjArr(self.value, ret.as_mut_ptr());
            drop(l);
            ret.set_len(b);

            // NOTE: This relies on the fact that BQNValue has the same in-memory representation
            // than u64 (BQNV)
            std::mem::transmute::<Vec<u64>, Vec<BQNValue>>(ret)
        }
    }

    fn into_char_container<T: FromIterator<char>>(self) -> T {
        let l = LOCK.lock();
        if !bqneltype_is_char(self.direct_arr_type()) {
            panic!("value isn't a character array");
        }

        let b = self.bound();
        let mut u32s = Vec::with_capacity(b);
        unsafe {
            bqn_readC32Arr(self.value, u32s.as_mut_ptr());
            drop(l);
            u32s.set_len(b);
        }

        u32s.into_iter().filter_map(char::from_u32).collect::<T>()
    }

    pub fn into_char_vec(self) -> Vec<char> {
        self.into_char_container::<Vec<char>>()
    }

    pub fn into_string(self) -> String {
        self.into_char_container::<String>()
    }

    fn bound(&self) -> usize {
        unsafe { bqn_bound(self.value) as usize }
    }

    fn bqn_type(&self) -> BQNType {
        BQNType::try_from(unsafe { bqn_type(self.value) }).expect("expected to handle all types")
    }

    fn direct_arr_type(&self) -> u32 {
        unsafe { bqn_directArrType(self.value) }
    }
}

#[derive(Debug, PartialEq)]
enum BQNType {
    Array,
    Number,
    Character,
    Function,
    Mod1,
    Mod2,
    Namespace,
}
impl TryFrom<i32> for BQNType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BQNType::Array),
            1 => Ok(BQNType::Number),
            2 => Ok(BQNType::Character),
            3 => Ok(BQNType::Function),
            4 => Ok(BQNType::Mod1),
            5 => Ok(BQNType::Mod2),
            6 => Ok(BQNType::Namespace),
            _ => Err("Invalid type"),
        }
    }
}

const fn bqneltype_is_numeric(eltype: u32) -> bool {
    #![allow(non_upper_case_globals)]
    match eltype {
        BQNElType_elt_f64 | BQNElType_elt_i32 | BQNElType_elt_i16 | BQNElType_elt_i8 => true,
        _ => false,
    }
}

const fn bqneltype_is_char(eltype: u32) -> bool {
    #![allow(non_upper_case_globals)]
    match eltype {
        BQNElType_elt_c32 | BQNElType_elt_c16 | BQNElType_elt_c8 => true,
        _ => false,
    }
}

const fn bqneltype_is_unknown(eltype: u32) -> bool {
    eltype == BQNElType_elt_unk
}

impl fmt::Debug for BQNValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = crate::eval("•Fmt");
        let formatted = fmt.call1(self);
        write!(f, "{}", formatted.into_string())
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
    use std::panic;

    fn c8_str() -> BQNValue {
        BQN!(r#""hello""#)
    }

    fn c16_str() -> BQNValue {
        BQN!(r#""hello∘""#)
    }

    fn c32_str() -> BQNValue {
        BQN!(r#""hello💣""#)
    }

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
        assert_eq!(ret.into_f64_vec(), vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn into_i32_vec() {
        let ret = eval("0.25+↕5");
        assert_eq!(ret.into_i32_vec(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn into_bqnvalue_vec() {
        let strings = BQN!("↑", "hello")
            .into_bqnvalue_vec()
            .into_iter()
            .map(|v| v.into_string())
            .collect::<Vec<String>>();
        assert_eq!(strings, vec!["", "h", "he", "hel", "hell", "hello"]);
    }

    #[test]
    fn call1() {
        let f = eval("↕");
        let ret = f.call1(&5.into());
        assert_eq!(ret.into_i32_vec(), vec![0, 1, 2, 3, 4]);
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
        assert_eq!(BQN!(2, "×", arr).into_i32_vec(), vec![2, 6, 12]);
    }

    #[test]
    fn test_debug_repr() {
        let v = BQN!("1‿2‿3");
        assert_eq!(format!("{:?}", v), "⟨ 1 2 3 ⟩");
    }

    #[test]
    fn should_panic_c8_string_to_f64_vec() {
        let v = c8_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
        assert!(panic::catch_unwind(move || {
            _ = v.into_f64_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c16_string_to_f64_vec() {
        let v = c16_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
        assert!(panic::catch_unwind(move || {
            _ = v.into_f64_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c32_string_to_f64_vec() {
        let v = c32_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
        assert!(panic::catch_unwind(move || {
            _ = v.into_f64_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c8_string_to_i32_vec() {
        let v = c8_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
        assert!(panic::catch_unwind(move || {
            _ = v.into_i32_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c16_string_to_i32_vec() {
        let v = c16_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
        assert!(panic::catch_unwind(move || {
            _ = v.into_i32_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c32_string_to_i32_vec() {
        let v = c32_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
        assert!(panic::catch_unwind(move || {
            v.into_i32_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_f64_arr_to_string() {
        let v = BQN!("1.2‿3.4‿5.6");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_f64);
        assert!(panic::catch_unwind(move || {
            let _ = v.into_string();
        })
        .is_err());
    }

    #[test]
    fn should_panic_i32_arr_to_string() {
        let v = BQN!("67000‿68000");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_i32);
        assert!(panic::catch_unwind(move || {
            v.into_string();
        })
        .is_err());
    }

    #[test]
    fn should_panic_i16_arr_to_string() {
        let v = BQN!("1234‿5678");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_i16);
        assert!(panic::catch_unwind(move || {
            v.into_string();
        })
        .is_err());
    }

    #[test]
    fn should_panic_i8_arr_to_string() {
        let v = BQN!("12‿34");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_i8);
        assert!(panic::catch_unwind(move || {
            v.into_string();
        })
        .is_err());
    }

    #[test]
    fn should_panic_number_to_string() {
        let v = BQN!("123");
        assert!(panic::catch_unwind(move || {
            v.into_string();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c8_string_to_bqnvalue_vec() {
        let v = c8_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c8);
        assert!(panic::catch_unwind(move || {
            _ = v.into_bqnvalue_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c16_string_to_bqnvalue_vec() {
        let v = c16_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c16);
        assert!(panic::catch_unwind(move || {
            _ = v.into_bqnvalue_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_c32_string_to_bqnvalue_vec() {
        let v = c32_str();
        assert_eq!(v.direct_arr_type(), BQNElType_elt_c32);
        assert!(panic::catch_unwind(move || {
            _ = v.into_bqnvalue_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_f64_arr_to_bqnvalue_vec() {
        let v = BQN!("1.2‿3.4‿5.6");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_f64);
        assert!(panic::catch_unwind(move || {
            let _ = v.into_bqnvalue_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_i32_arr_to_bqnvalue_vec() {
        let v = BQN!("67000‿68000");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_i32);
        assert!(panic::catch_unwind(move || {
            v.into_bqnvalue_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_i16_arr_to_bqnvalue_vec() {
        let v = BQN!("1234‿5678");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_i16);
        assert!(panic::catch_unwind(move || {
            v.into_bqnvalue_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_i8_arr_to_bqnvalue_vec() {
        let v = BQN!("12‿34");
        assert_eq!(v.direct_arr_type(), BQNElType_elt_i8);
        assert!(panic::catch_unwind(move || {
            v.into_bqnvalue_vec();
        })
        .is_err());
    }

    #[test]
    fn should_panic_number_to_bqnvalue_vec() {
        let v = BQN!("123");
        assert!(panic::catch_unwind(move || {
            v.into_bqnvalue_vec();
        })
        .is_err());
    }
}
