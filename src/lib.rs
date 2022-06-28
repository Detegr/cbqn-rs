use cbqn_sys::*;
use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use std::{fmt, sync::Once};

#[cfg(test)]
mod tests;

mod conversions;
mod macros;
mod typecheck;

use typecheck::*;

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
