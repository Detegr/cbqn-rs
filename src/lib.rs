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
//! assert_eq!(sum.to_f64(), 2.0);
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! assert_eq!(BQN!("⌽≡⊢", "BQN").to_f64(), 0.0);
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! let strs = BQN!(' ', "(⊢-˜+`×¬)∘=⊔⊢", "Rust ❤️ BQN")
//!     .to_bqnvalue_vec()
//!     .iter()
//!     .map(BQNValue::to_string)
//!     .collect::<Vec<String>>();
//! assert_eq!(strs, ["Rust", "❤️", "BQN"]);
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval};
//! let strings = ["join", "these", "please"];
//! assert_eq!(BQN!("∾", strings).to_string(), "jointheseplease");
//! ```
//!
//! # Examples using BQNValue
//!
//! ```
//! # use cbqn::{BQNValue, eval};
//! let sum = eval("1+1");
//! assert_eq!(sum.to_f64(), 2.0);
//! ```
//!
//! ```
//! # use cbqn::{BQNValue, eval};
//! let is_anagram = eval("⌽≡⊢");
//! assert_eq!(is_anagram.call1(&"BQN".into()).to_f64(), 0.0);
//! ```
//!
//! ```
//! # use cbqn::{BQNValue, eval};
//! let split = eval("(⊢-˜+`×¬)∘=⊔⊢");
//! let strs = split.call2(&' '.into(), &"Rust ❤️ BQN".into())
//!     .to_bqnvalue_vec()
//!     .iter()
//!     .map(BQNValue::to_string)
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
//! assert_eq!(result.to_f64(), 6.0);
//! ```
use cbqn_sys::*;
use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use rand::{self, prelude::*};
use std::{cell::RefCell, collections::HashMap, fmt, mem, sync::Once};

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
    value: BQNV,
    boundfn_key: u64,
}

impl BQNValue {
    fn new(value: BQNV) -> BQNValue {
        BQNValue {
            value,
            boundfn_key: 0,
        }
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
    pub fn to_f64(&self) -> f64 {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Number {
            panic!("value isn't a number");
        }
        unsafe { bqn_readF64(self.value) }
    }

    /// Converts `BQNValue` into `char`
    ///
    /// Returns `None` if the value is not an Unicode scalar value. Rust `char`s cannot represent
    /// characters that are not Unicode scalar values.
    ///
    /// # Panics
    /// * If `self` isn't a BQN character
    pub fn to_char(&self) -> Option<char> {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            panic!("value isn't a character");
        }
        unsafe { char::from_u32(bqn_readChar(self.value)) }
    }

    /// Converts `BQNValue` into `u32`
    ///
    /// BQN characters can contain values that aren't Unicode scalar values. Those characters can
    /// be converted into a Rust type `u32` using this function.
    ///
    /// # Panics
    /// * If `self` isn't a BQN character
    pub fn to_u32(&self) -> u32 {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            panic!("value isn't a character");
        }
        unsafe { bqn_readChar(self.value) }
    }

    /// Converts `BQNValue` into a vector of `f64`s
    ///
    /// # Panics
    /// * If `self` isn't a BQN array containing numbers
    pub fn to_f64_vec(&self) -> Vec<f64> {
        let l = LOCK.lock();
        let b = self.get_numeric_array_bounds_or_panic();
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
    pub fn to_i32_vec(&self) -> Vec<i32> {
        let l = LOCK.lock();
        let b = self.get_numeric_array_bounds_or_panic();
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
    pub fn to_bqnvalue_vec(&self) -> Vec<BQNValue> {
        let l = LOCK.lock();
        if !bqneltype_is_unknown(self.direct_arr_type()) {
            panic!("value isn't an object array");
        } else {
            if self.bqn_type() != BQNType::Array {
                panic!("value isn't an object array");
            }
        }

        let b = self.bound();
        let mut objarr = Vec::with_capacity(b);
        unsafe {
            bqn_readObjArr(self.value, objarr.as_mut_ptr());
            drop(l);
            objarr.set_len(b);
        }

        objarr.into_iter().map(BQNValue::new).collect()
    }

    fn to_char_container<T: FromIterator<char>>(&self) -> T {
        let l = LOCK.lock();
        let b = self.get_character_array_bounds_or_panic();
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
    pub fn to_char_vec(&self) -> Vec<char> {
        self.to_char_container::<Vec<char>>()
    }

    /// Converts `BQNValue` into a `String`
    ///
    /// # Panics
    /// * If `self` isn't a BQN array that contains characters
    pub fn to_string(&self) -> String {
        self.to_char_container::<String>()
    }

    /// Generates a BQNValue from a Rust function
    ///
    /// The function receives one argument.
    ///
    /// # Examples
    /// ```
    /// # use cbqn::{BQN, BQNValue, eval};
    /// let add_three = BQNValue::fn1(|x| BQNValue::from(x.to_f64() + 3.0));
    /// assert_eq!(BQN!(3, "{𝕏𝕨}", add_three).to_f64(), 6.0);
    /// ```
    pub fn fn1<F: Fn(&BQNValue) -> BQNValue + 'static>(f: F) -> BQNValue {
        INIT.call_once(|| {
            let _l = LOCK.lock();
            unsafe { bqn_init() }
        });

        let mut rng = rand::thread_rng();
        let mut key = rng.gen::<u64>() & 0xFFFFFFFF;
        FNS.with(|fns| {
            let mut boundfns = fns.borrow_mut();
            // unlikely
            while boundfns.boundfn_1.contains_key(&key) || key == 0 {
                key = rng.gen::<u64>() & 0xFFFFFFFF;
            }
            boundfns.boundfn_1.insert(key, Box::new(f));
        });

        let obj = BQNValue::from(f64::from_bits(key));
        let _l = LOCK.lock();
        BQNValue {
            value: unsafe { bqn_makeBoundFn1(Some(boundfn_1_wrapper), obj.value) },
            boundfn_key: key,
        }
    }

    /// Generates a BQNValue from a Rust function
    ///
    /// The function receives two arguments.
    ///
    /// # Examples
    /// ```
    /// # use cbqn::{BQN, BQNValue, eval};
    /// let multiply = BQNValue::fn2(|w, x| BQNValue::from(w.to_f64() * x.to_f64()));
    /// assert_eq!(BQN!(multiply, "{𝕎´𝕩}", [1,2,3,4,5]).to_f64(), 120.0);
    /// ```
    pub fn fn2<F: Fn(&BQNValue, &BQNValue) -> BQNValue + 'static>(f: F) -> BQNValue {
        INIT.call_once(|| {
            let _l = LOCK.lock();
            unsafe { bqn_init() }
        });

        let mut rng = rand::thread_rng();
        let mut key = rng.gen::<u64>() & 0xFFFFFFFF00000000;
        FNS.with(|fns| {
            let mut boundfns = fns.borrow_mut();
            // unlikely
            while boundfns.boundfn_2.contains_key(&key) || key == 0 {
                key = rng.gen::<u64>() & 0xFFFFFFFF00000000;
            }
            boundfns.boundfn_2.insert(key, Box::new(f));
        });

        let obj = BQNValue::from(f64::from_bits(key));
        let _l = LOCK.lock();
        BQNValue {
            value: unsafe { bqn_makeBoundFn2(Some(boundfn_2_wrapper), obj.value) },
            boundfn_key: key,
        }
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

    fn get_character_array_bounds_or_panic(&self) -> usize {
        if self.bqn_type() != BQNType::Array {
            panic!("value isn't an array");
        }
        let b = self.bound();
        if !bqneltype_is_char(self.direct_arr_type()) {
            for i in 0..b {
                let t = BQNType::try_from(unsafe {
                    let v = bqn_pick(self.value, i.try_into().unwrap());
                    let t = bqn_type(v);
                    bqn_free(v);
                    t
                })
                .expect("expected known type");

                if t != BQNType::Character {
                    panic!("value isn't a character array");
                }
            }
        }
        b
    }

    fn get_numeric_array_bounds_or_panic(&self) -> usize {
        if self.bqn_type() != BQNType::Array {
            panic!("value isn't an array");
        }
        let b = self.bound();
        if !bqneltype_is_numeric(self.direct_arr_type()) {
            for i in 0..b {
                let t = BQNType::try_from(unsafe {
                    let v = bqn_pick(self.value, i.try_into().unwrap());
                    let t = bqn_type(v);
                    bqn_free(v);
                    t
                })
                .expect("expected known type");

                if t != BQNType::Number {
                    panic!("value isn't a numeric array");
                }
            }
        }
        b
    }
}

impl fmt::Debug for BQNValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = crate::eval("•Fmt");
        let formatted = fmt.call1(self);
        write!(f, "{}", formatted.to_string())
    }
}

impl Drop for BQNValue {
    fn drop(&mut self) {
        let _l = LOCK.lock();
        unsafe { bqn_free(self.value) };
        if self.boundfn_key > 0 {
            FNS.with(|fns| {
                if self.boundfn_key <= 0xFFFFFFFF {
                    (*fns.borrow_mut()).boundfn_1.remove(&self.boundfn_key);
                } else {
                    (*fns.borrow_mut()).boundfn_2.remove(&self.boundfn_key);
                }
            })
        }
    }
}

#[derive(Default)]
struct BoundFns {
    boundfn_1: HashMap<u64, Box<dyn Fn(&BQNValue) -> BQNValue>>,
    boundfn_2: HashMap<u64, Box<dyn Fn(&BQNValue, &BQNValue) -> BQNValue>>,
}

thread_local! {
    static FNS: RefCell<BoundFns> = RefCell::new(BoundFns::default());
}

unsafe extern "C" fn boundfn_1_wrapper(obj: BQNV, x: BQNV) -> BQNV {
    let key = BQNValue::new(obj).to_f64().to_bits();

    let tgt = FNS.with(|fns| {
        let mut boundfns = fns.borrow_mut();
        boundfns.boundfn_1.remove(&key).unwrap()
    });

    let l = LOCK.lock();
    let ret = tgt(&BQNValue::new(x));
    drop(l);

    FNS.with(|fns| {
        let mut boundfns = fns.borrow_mut();
        boundfns.boundfn_1.insert(key, Box::new(tgt));
    });

    let retval = ret.value;
    mem::forget(ret);
    retval
}

unsafe extern "C" fn boundfn_2_wrapper(obj: BQNV, w: BQNV, x: BQNV) -> BQNV {
    let key = BQNValue::new(obj).to_f64().to_bits();

    let tgt = FNS.with(|fns| {
        let mut boundfns = fns.borrow_mut();
        boundfns.boundfn_2.remove(&key).unwrap()
    });

    let l = LOCK.lock();
    let ret = tgt(&BQNValue::new(w), &BQNValue::new(x));
    drop(l);

    FNS.with(|fns| {
        let mut boundfns = fns.borrow_mut();
        boundfns.boundfn_2.insert(key, Box::new(tgt));
    });

    let retval = ret.value;
    mem::forget(ret);
    retval
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
