//! cbqn
//!
//! A crate for running [BQN](https://mlochbaum.github.io/BQN) code within a Rust program using [CBQN](https://github.com/dzaima/CBQN) interpreter compiled as a shared object.
//!
//! # Usage
//!
//! Simple expressions can be run with the `BQN!` convenience macro. For more advanced use, the
//! methods of `BQNValue` provide the necessary functionality.
//!
//! # Examples using the BQN! macro
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! let sum = BQN!("1+1");
//! assert_eq!(sum.into_f64(), 2.0);
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! assert_eq!(BQN!("⌽≡⊢", "BQN").into_f64(), 0.0);
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! let strs = BQN!(' ', "(⊢-˜+`×¬)∘=⊔⊢", "Rust ❤️ BQN")
//!     .into_bqnvalue_vec()
//!     .into_iter()
//!     .map(BQNValue::into_string)
//!     .collect::<Vec<String>>();
//! assert_eq!(strs, ["Rust", "❤️", "BQN"]);
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! let strings = ["join", "these", "please"];
//! assert_eq!(BQN!("∾", strings).into_string(), "jointheseplease");
//! ```
//!
//! # Examples using BQNValue
//!
//! ```
//! # use cbqn::{BQNValue, eval};
//! let sum = eval("1+1");
//! assert_eq!(sum.into_f64(), 2.0);
//! ```
//!
//! ```
//! # use cbqn::{BQNValue, eval};
//! let is_anagram = eval("⌽≡⊢");
//! assert_eq!(is_anagram.call1(&"BQN".into()).into_f64(), 0.0);
//! ```
//!
//! ```
//! # use cbqn::{BQNValue, eval};
//! let split = eval("(⊢-˜+`×¬)∘=⊔⊢");
//! let strs = split.call2(&' '.into(), &"Rust ❤️ BQN".into())
//!     .into_bqnvalue_vec()
//!     .into_iter()
//!     .map(BQNValue::into_string)
//!     .collect::<Vec<String>>();
//! assert_eq!(strs, ["Rust", "❤️", "BQN"]);
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! let counter = BQN!("{v←0 ⋄ Inc⇐{v+↩𝕩}}");
//! let increment = counter.get_field("inc").unwrap();
//! increment.call1(&1.into());
//! increment.call1(&2.into());
//! let result = increment.call1(&3.into());
//! assert_eq!(result.into_f64(), 6.0);
//! ```
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

/// Represents a BQN value
pub struct BQNValue {
    // Fields of this struct must not be altered.
    // It has to have the same in-memory representation than plain BQNV
    value: BQNV,
}

impl BQNValue {
    fn new(value: BQNV) -> BQNValue {
        BQNValue { value }
    }

    /// Constructs a BQN null value `@`
    ///
    /// # Examples
    ///
    /// ```
    /// # use cbqn::{BQN, BQNValue, eval};
    /// BQN!('a', "-", BQNValue::null());
    /// ```
    pub fn null() -> BQNValue {
        let _l = LOCK.lock();
        crate::INIT.call_once(|| unsafe { bqn_init() });
        BQNValue::new(unsafe { bqn_makeChar(0) })
    }

    /// Returns a boolean value indicating whether `field` exists in a BQN namespace
    /// As CBQN requires the searched field name to be in a string with all lowercase letters, this
    /// function returns `false` if it is supplied with a `field` string that contains uppercase
    /// characters.
    ///
    /// # Panics
    ///
    /// * If `self` isn't a namespace.
    pub fn has_field(&self, field: &str) -> bool {
        let _l = LOCK.lock();

        // CBQN requires the string to be all lowercase
        if !field.chars().all(char::is_lowercase) {
            return false;
        }

        if self.bqn_type() != BQNType::Namespace {
            panic!("value isn't a namespace");
        }
        unsafe { bqn_hasField(self.value, BQNValue::from(field).value) }
    }

    /// Returns `field` from a BQN namespace as `BQNValue`. Returns `None` if the field cannot be
    /// found.
    /// As CBQN requires the searched field name to be in a string with all lowercase letters, this
    /// function returns `None` if it is supplied with a `field` string that contains uppercase
    /// characters.
    ///
    /// # Panics
    ///
    /// * If `self` isn't a namespace.
    pub fn get_field(&self, field: &str) -> Option<BQNValue> {
        let _l = LOCK.lock();

        // CBQN requires the string to be all lowercase
        if !field.chars().all(char::is_lowercase) {
            return None;
        }

        if self.bqn_type() != BQNType::Namespace {
            panic!("value isn't a namespace");
        }
        let f = BQNValue::from(field);
        unsafe {
            if bqn_hasField(self.value, f.value) {
                Some(BQNValue::new(bqn_getField(self.value, f.value)))
            } else {
                None
            }
        }
    }

    /// Calls `BQNValue` as a function with one argument
    pub fn call1(&self, x: &BQNValue) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue::new(unsafe { bqn_call1(self.value, x.value) })
    }

    /// Calls `BQNValue` as a function with two arguments
    pub fn call2(&self, w: &BQNValue, x: &BQNValue) -> BQNValue {
        let _l = LOCK.lock();
        unsafe { BQNValue::new(bqn_call2(self.value, w.value, x.value)) }
    }

    /// Converts `BQNValue` into `f64`
    ///
    /// # Panics
    /// * If `self` isn't a number
    pub fn into_f64(self) -> f64 {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Number {
            panic!("value isn't a number");
        }
        unsafe { bqn_toF64(self.value) }
    }

    /// Converts `BQNValue` into `char`
    ///
    /// Returns `None` if the value is not an Unicode scalar value. Rust `char`s cannot represent
    /// characters that are not Unicode scalar values.
    ///
    /// # Panics
    /// * If `self` isn't a BQN character
    pub fn into_char(self) -> Option<char> {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            panic!("value isn't a character");
        }
        unsafe { char::from_u32(bqn_toChar(self.value)) }
    }

    /// Converts `BQNValue` into `u32`
    ///
    /// BQN characters can contain values that aren't Unicode scalar values. Those characters can
    /// be converted into a Rust type `u32` using this function.
    ///
    /// # Panics
    /// * If `self` isn't a BQN character
    pub fn into_u32(self) -> u32 {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            panic!("value isn't a character");
        }
        unsafe { bqn_toChar(self.value) }
    }

    /// Converts `BQNValue` into a vector of `f64`s
    ///
    /// # Panics
    /// * If `self` isn't a BQN array containing numbers
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

    /// Converts `BQNValue` into a vector of `i32`s
    ///
    /// This function will do a lossy conversion from `f64` to `i32` for the values of the array.
    ///
    /// # Panics
    /// * If `self` isn't a BQN array containing numbers
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

    /// Converts `BQNValue` into a vector of `BQNValue`s
    ///
    /// # Panics
    /// * If `self` isn't a BQN array that contains BQN objects
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

    /// Converts `BQNValue` into vector of `char`s
    ///
    /// # Panics
    /// * If `self` isn't a BQN array that contains characters
    pub fn into_char_vec(self) -> Vec<char> {
        self.into_char_container::<Vec<char>>()
    }

    /// Converts `BQNValue` into a `String`
    ///
    /// # Panics
    /// * If `self` isn't a BQN array that contains characters
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
///
/// # Examples
/// ```
/// # use cbqn::eval;
/// let bqnv = eval("1+1");
/// let bqnfn = eval("{𝕩×10}");
/// ```
pub fn eval(bqn: &str) -> BQNValue {
    INIT.call_once(|| {
        let _l = LOCK.lock();
        unsafe { bqn_init() }
    });
    let _l = LOCK.lock();
    let ret = BQNValue::new(unsafe { bqn_eval(BQNValue::from(bqn).value) });
    ret
}
