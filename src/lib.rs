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
use std::{cell::RefCell, fmt, mem, sync::Once};

#[cfg(test)]
mod tests;

mod bqntype;
mod conversions;
mod macros;

pub use bqntype::BQNType;

static LOCK: Lazy<ReentrantMutex<()>> = Lazy::new(|| ReentrantMutex::new(()));
static INIT: Once = Once::new();

/// Represents a BQN value
pub struct BQNValue {
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

        if !BQNValue::is_valid_namespace_field(field) {
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

        if !BQNValue::is_valid_namespace_field(field) {
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

    /// Returns the BQN type of the BQNValue
    pub fn bqn_type(&self) -> BQNType {
        BQNType::try_from(unsafe { bqn_type(self.value) }).expect("expected to handle all types")
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

    /// Converts `BQNValue` into a vector of `BQNValue`s
    ///
    /// # Panics
    /// * If `self` isn't a BQN array that contains BQN objects
    pub fn to_bqnvalue_vec(&self) -> Vec<BQNValue> {
        let l = LOCK.lock();
        if self.bqn_type() != BQNType::Array {
            panic!("value isn't an object array");
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

    fn is_valid_namespace_field(field: &str) -> bool {
        // CBQN requires the string to be all lowercase and contain no underscores
        field.chars().all(|c| c.is_lowercase() && c != '_')
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
    ///
    /// # Panics
    ///
    /// * If called from a BQNValue::fn1 or BQNValue::fn2 function
    ///
    /// # Implementation note
    ///
    /// Calling this function will allocate memory that will last for the lifetime of the program.
    /// Calling it with two identical closures, but with different lifetimes, will allocate the
    /// memory multiple times.
    pub fn fn1(func: fn(&BQNValue) -> BQNValue) -> BQNValue {
        INIT.call_once(|| {
            let _l = LOCK.lock();
            unsafe { bqn_init() }
        });

        let mut key = 0;
        FNS.with(|fns| {
            let mut boundfns = fns.borrow_mut();
            let mut exists = false;
            for f in boundfns.boundfn_1.iter() {
                if *f as usize == func as usize {
                    exists = true;
                    break;
                }
            }
            if !exists {
                boundfns.boundfn_1.push(func);
                key = boundfns.boundfn_1.len() as u64 - 1;
            }
        });

        let obj = BQNValue::from(f64::from_bits(key));
        let _l = LOCK.lock();
        BQNValue {
            value: unsafe { bqn_makeBoundFn1(Some(boundfn_1_wrapper), obj.value) },
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
    ///
    /// # Panics
    ///
    /// * If called from a BQNValue::fn1 or BQNValue::fn2 function
    ///
    /// # Implementation note
    ///
    /// Calling this function will allocate memory that will last for the lifetime of the program.
    /// Calling it with two identical closures, but with different lifetimes, will allocate the
    /// memory multiple times.
    pub fn fn2(func: fn(&BQNValue, &BQNValue) -> BQNValue) -> BQNValue {
        INIT.call_once(|| {
            let _l = LOCK.lock();
            unsafe { bqn_init() }
        });

        let mut key = 0;
        FNS.with(|fns| {
            let mut boundfns = fns.borrow_mut();
            let mut exists = false;
            for f in boundfns.boundfn_2.iter() {
                if *f as usize == func as usize {
                    exists = true;
                    break;
                }
            }
            if !exists {
                boundfns.boundfn_2.push(func);
                key = boundfns.boundfn_2.len() as u64 - 1;
            }
        });

        let obj = BQNValue::from(f64::from_bits(key));
        let _l = LOCK.lock();
        BQNValue {
            value: unsafe { bqn_makeBoundFn2(Some(boundfn_2_wrapper), obj.value) },
        }
    }

    fn bound(&self) -> usize {
        unsafe { bqn_bound(self.value) as usize }
    }

    fn direct_arr_type(&self) -> u32 {
        unsafe { bqn_directArrType(self.value) }
    }

    fn get_character_array_bounds_or_panic(&self) -> usize {
        if self.bqn_type() != BQNType::Array {
            panic!("value isn't an array");
        }
        let b = self.bound();
        if !self.known_char_arr() {
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
        if !self.known_f64_arr() {
            for i in 0..b {
                let t = BQNType::try_from(unsafe {
                    let v = bqn_pick(self.value, i.try_into().unwrap());
                    let t = bqn_type(v);
                    bqn_free(v);
                    t
                })
                .expect("expected known type");

                if t != BQNType::Number {
                    panic!("value isn't a f64 array");
                }
            }
        }
        b
    }

    // This function returns whether it's known that the array elements are of type f64
    // Returns false if *it is not known* whether the array elements are of type f64 or not
    fn known_f64_arr(&self) -> bool {
        #![allow(non_upper_case_globals)]
        match self.direct_arr_type() {
            BQNElType_elt_f64 | BQNElType_elt_i32 | BQNElType_elt_i16 | BQNElType_elt_i8 => true,
            _ => false,
        }
    }

    // This function returns whether it's known that the array elements are of type char
    // Returns false if *it is not known* whether the array elements are of type char or not
    fn known_char_arr(&self) -> bool {
        #![allow(non_upper_case_globals)]
        match self.direct_arr_type() {
            BQNElType_elt_c32 | BQNElType_elt_c16 | BQNElType_elt_c8 => true,
            _ => false,
        }
    }
}

impl fmt::Debug for BQNValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = crate::eval("•Fmt");
        let formatted = fmt.call1(self);
        write!(f, "{}", formatted.to_string())
    }
}

impl Clone for BQNValue {
    fn clone(&self) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue {
            value: unsafe { bqn_copy(self.value) },
        }
    }
}

impl Drop for BQNValue {
    fn drop(&mut self) {
        let _l = LOCK.lock();
        unsafe { bqn_free(self.value) };
    }
}

#[derive(Default)]
pub(crate) struct BoundFns {
    boundfn_1: Vec<fn(&BQNValue) -> BQNValue>,
    boundfn_2: Vec<fn(&BQNValue, &BQNValue) -> BQNValue>,
}

thread_local! {
    static FNS: RefCell<BoundFns> = RefCell::new(BoundFns::default());
}

unsafe extern "C" fn boundfn_1_wrapper(obj: BQNV, x: BQNV) -> BQNV {
    let key = BQNValue::new(obj).to_f64().to_bits() as usize;

    FNS.with(|fns| {
        let boundfns = fns.borrow();
        let tgt = &boundfns.boundfn_1[key];
        let ret = tgt(&BQNValue::new(x));
        let retval = ret.value;
        mem::forget(ret);
        retval
    })
}

unsafe extern "C" fn boundfn_2_wrapper(obj: BQNV, w: BQNV, x: BQNV) -> BQNV {
    let key = BQNValue::new(obj).to_f64().to_bits() as usize;

    FNS.with(|fns| {
        let boundfns = fns.borrow();
        let tgt = &boundfns.boundfn_2[key];
        let ret = tgt(&BQNValue::new(w), &BQNValue::new(x));
        let retval = ret.value;
        mem::forget(ret);
        retval
    })
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
