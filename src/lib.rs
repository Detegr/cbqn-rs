// Copyright (C) 2022-2023 Antti Keränen
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 3.

//! cbqn
//!
//! A crate for running [BQN](https://mlochbaum.github.io/BQN) code within a Rust program using [CBQN](https://github.com/dzaima/CBQN) interpreter compiled as a shared object or a WASI reactor.
//!
//! # Usage
//!
//! Simple expressions can be run with the `BQN!` convenience macro. For more advanced use, the
//! methods of `BQNValue` provide the necessary functionality.
//!
//! # Examples using the BQN! macro
//! ```
//! # use cbqn::{BQN, BQNValue, eval, Error};
//! let sum = BQN!("1+1")?;
//! assert_eq!(sum.to_f64()?, 2.0);
//! # Ok::<(), Error>(())
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval, Error};
//! assert_eq!(BQN!("⌽≡⊢", "BQN")?.to_f64()?, 0.0);
//! # Ok::<(), Error>(())
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval, Error};
//! let strs = BQN!(' ', "(⊢-˜+`×¬)∘=⊔⊢", "Rust ❤️ BQN")?
//!     .to_bqnvalue_vec()?
//!     .iter()
//!     .map(|v| v.to_string())
//!     .collect::<Result<Vec<String>, _>>()?;
//! assert_eq!(strs, ["Rust", "❤️", "BQN"]);
//! # Ok::<(), Error>(())
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval, Error};
//! let strings = ["join", "these", "please"];
//! assert_eq!(BQN!("∾", strings)?.to_string()?, "jointheseplease");
//! # Ok::<(), Error>(())
//! ```
//!
//! # Examples using BQNValue
//!
//! ```
//! # use cbqn::{BQNValue, eval, Error};
//! let sum = eval("1+1")?;
//! assert_eq!(sum.to_f64()?, 2.0);
//! # Ok::<(), Error>(())
//! ```
//!
//! ```
//! # use cbqn::{BQNValue, eval, Error};
//! let is_palindrome = eval("⌽≡⊢")?;
//! assert_eq!(is_palindrome.call1(&"BQN".into())?.to_f64()?, 0.0);
//! # Ok::<(), Error>(())
//! ```
//!
//! ```
//! # use cbqn::{BQNValue, eval, Error};
//! let split = eval("(⊢-˜+`×¬)∘=⊔⊢")?;
//! let strs = split.call2(&' '.into(), &"Rust ❤️ BQN".into())?
//!     .to_bqnvalue_vec()?
//!     .iter()
//!     .map(BQNValue::to_string)
//!     .collect::<Result<Vec<String>, _>>()?;
//! assert_eq!(strs, ["Rust", "❤️", "BQN"]);
//! # Ok::<(), Error>(())
//! ```
//!
//! ```
//! # use cbqn::{BQN, BQNValue, eval, Error};
//! let counter = BQN!("{v←0 ⋄ Inc⇐{v+↩𝕩}}")?;
//! let increment = counter.get_field("inc")?.unwrap();
//! increment.call1(&1.into())?;
//! increment.call1(&2.into())?;
//! let result = increment.call1(&3.into())?;
//! assert_eq!(result.to_f64()?, 6.0);
//! # Ok::<(), Error>(())
//! ```

// Clippy outputs false positives because some of the casts and conversions are needed in the wasi
// backend, but not in the native backend
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::unit_arg)]

mod backend;
use backend::*;

use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use std::{cell::RefCell, fmt, mem, sync::Once};

#[cfg(test)]
mod tests;

mod bqntype;
mod conversions;
mod macros;

pub use backend::Error;
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
    /// BQN!('a', "-", BQNValue::null()).unwrap();
    /// ```
    pub fn null() -> BQNValue {
        let _l = LOCK.lock();
        crate::INIT.call_once(|| bqn_init().unwrap());
        BQNValue::new(bqn_makeChar(0).unwrap())
    }

    /// Returns a boolean value indicating whether `field` exists in a BQN namespace
    /// As CBQN requires the searched field name to be in a string with all lowercase letters, this
    /// function returns `false` if it is supplied with a `field` string that contains uppercase
    /// characters.
    pub fn has_field(&self, field: &str) -> Result<bool> {
        let _l = LOCK.lock();

        if !BQNValue::is_valid_namespace_field(field) {
            return Ok(false);
        }

        if self.bqn_type() != BQNType::Namespace {
            return Err(Error::InvalidType("value isn't a namespace".into()));
        }
        bqn_hasField(self.value, BQNValue::from(field).value)
    }

    /// Returns `field` from a BQN namespace as `BQNValue`. Returns `None` if the field cannot be
    /// found.
    /// As CBQN requires the searched field name to be in a string with all lowercase letters, this
    /// function returns `None` if it is supplied with a `field` string that contains uppercase
    /// characters.
    pub fn get_field(&self, field: &str) -> Result<Option<BQNValue>> {
        let _l = LOCK.lock();

        if !BQNValue::is_valid_namespace_field(field) {
            return Ok(None);
        }

        if self.bqn_type() != BQNType::Namespace {
            return Err(Error::InvalidType("value isn't a namespace".into()));
        }
        let f = BQNValue::from(field);
        Ok(if bqn_hasField(self.value, f.value).unwrap() {
            Some(BQNValue::new(bqn_getField(self.value, f.value)?))
        } else {
            None
        })
    }

    /// Calls `BQNValue` as a function with one argument
    pub fn call1(&self, x: &BQNValue) -> Result<BQNValue> {
        let _l = LOCK.lock();
        Ok(BQNValue::new(bqn_call1(self.value, x.value)?))
    }

    /// Calls `BQNValue` as a function with two arguments
    pub fn call2(&self, w: &BQNValue, x: &BQNValue) -> Result<BQNValue> {
        let _l = LOCK.lock();
        Ok(BQNValue::new(bqn_call2(self.value, w.value, x.value)?))
    }

    /// Returns the BQN type of the BQNValue
    pub fn bqn_type(&self) -> BQNType {
        BQNType::try_from(bqn_type(self.value).unwrap()).expect("expected to handle all types")
    }

    /// Converts `BQNValue` into `f64`
    pub fn to_f64(&self) -> Result<f64> {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Number {
            return Err(Error::InvalidType("value isn't a number".into()));
        }
        bqn_readF64(self.value)
    }

    /// Converts `BQNValue` into `char`
    ///
    /// Returns `None` if the value is not an Unicode scalar value. Rust `char`s cannot represent
    /// characters that are not Unicode scalar values.
    pub fn to_char(&self) -> Result<Option<char>> {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            return Err(Error::InvalidType("value isn't a character".into()));
        }
        Ok(char::from_u32(bqn_readChar(self.value)?))
    }

    /// Converts `BQNValue` into `u32`
    ///
    /// BQN characters can contain values that aren't Unicode scalar values. Those characters can
    /// be converted into a Rust type `u32` using this function.
    pub fn to_u32(&self) -> Result<u32> {
        let _l = LOCK.lock();
        if self.bqn_type() != BQNType::Character {
            return Err(Error::InvalidType("value isn't a character".into()));
        }
        bqn_readChar(self.value)
    }

    /// Converts `BQNValue` into a vector of `f64`s
    pub fn to_f64_vec(&self) -> Result<Vec<f64>> {
        let l = LOCK.lock();
        let b = self.get_numeric_array_bounds()?;
        let mut ret = Vec::with_capacity(b);
        #[allow(clippy::uninit_vec)]
        unsafe {
            // We need to set length beforehand as wasi backend will need the length
            // I don't want to mess with MaybeUninit<T> as it gets cumbersome and this is unsafe
            // anyway
            ret.set_len(b)
        };
        bqn_readF64Arr(self.value, &mut ret).unwrap();
        drop(l);

        Ok(ret)
    }

    /// Converts `BQNValue` into a vector of `BQNValue`s
    pub fn to_bqnvalue_vec(&self) -> Result<Vec<BQNValue>> {
        let l = LOCK.lock();
        if self.bqn_type() != BQNType::Array {
            return Err(Error::InvalidType("value isn't an object array".into()));
        }

        let b = self.bound();
        let mut objarr = Vec::with_capacity(b);
        #[allow(clippy::uninit_vec)]
        unsafe {
            // We need to set length beforehand as wasi backend will need the length
            // I don't want to mess with MaybeUninit<T> as it gets cumbersome and this is unsafe
            // anyway
            objarr.set_len(b)
        };
        bqn_readObjArr(self.value, &mut objarr).unwrap();
        drop(l);

        Ok(objarr.into_iter().map(BQNValue::new).collect())
    }

    fn is_valid_namespace_field(field: &str) -> bool {
        // CBQN requires the string to be all lowercase and contain no underscores
        field.chars().all(|c| c.is_lowercase() && c != '_')
    }

    fn to_char_container<T: FromIterator<char>>(&self) -> Result<T> {
        let l = LOCK.lock();
        let b = self.get_character_array_bounds()?;
        let mut u32s = Vec::with_capacity(b);
        #[allow(clippy::uninit_vec)]
        unsafe {
            // We need to set length beforehand as wasi backend will need the length
            // I don't want to mess with MaybeUninit<T> as it gets cumbersome and this is unsafe
            // anyway
            u32s.set_len(b)
        };
        bqn_readC32Arr(self.value, &mut u32s).unwrap();
        drop(l);

        Ok(u32s.into_iter().filter_map(char::from_u32).collect::<T>())
    }

    /// Converts `BQNValue` into vector of `char`s
    pub fn to_char_vec(&self) -> Result<Vec<char>> {
        self.to_char_container::<Vec<char>>()
    }

    /// Converts `BQNValue` into a `String`
    pub fn to_string(&self) -> Result<String> {
        self.to_char_container::<String>()
    }

    /// Generates a BQNValue from a Rust function
    ///
    /// The function receives one argument.
    ///
    /// # Examples
    /// ```
    /// # use cbqn::{BQN, BQNValue, eval};
    /// # #[cfg(not(feature = "wasi-backend"))]
    /// # {
    /// let add_three = BQNValue::fn1(|x| BQNValue::from(x.to_f64().unwrap() + 3.0));
    /// assert_eq!(BQN!(3, "{𝕏𝕨}", add_three).unwrap().to_f64().unwrap(), 6.0);
    /// # }
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
    ///
    /// # Backend support
    ///
    /// Not supported in WASI backend
    pub fn fn1(func: fn(&BQNValue) -> BQNValue) -> BQNValue {
        INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap();
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
            value: bqn_makeBoundFn1(Some(boundfn_1_wrapper), obj.value).unwrap(),
        }
    }

    /// Generates a BQNValue from a Rust function
    ///
    /// The function receives two arguments.
    ///
    /// # Examples
    /// ```
    /// # use cbqn::{BQN, BQNValue, eval};
    /// # #[cfg(not(feature = "wasi-backend"))]
    /// # {
    /// let multiply = BQNValue::fn2(|w, x| BQNValue::from(w.to_f64().unwrap() *
    /// x.to_f64().unwrap()));
    /// assert_eq!(BQN!(multiply, "{𝕎´𝕩}", [1,2,3,4,5]).unwrap().to_f64().unwrap(), 120.0);
    /// # }
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
    ///
    /// # Backend support
    ///
    /// Not supported in WASI backend
    pub fn fn2(func: fn(&BQNValue, &BQNValue) -> BQNValue) -> BQNValue {
        INIT.call_once(|| {
            let _l = LOCK.lock();
            bqn_init().unwrap()
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
            value: bqn_makeBoundFn2(Some(boundfn_2_wrapper), obj.value).unwrap(),
        }
    }

    fn bound(&self) -> usize {
        bqn_bound(self.value).unwrap() as usize
    }

    fn direct_arr_type(&self) -> u32 {
        bqn_directArrType(self.value).unwrap()
    }

    fn get_character_array_bounds(&self) -> Result<usize> {
        if self.bqn_type() != BQNType::Array {
            return Err(Error::InvalidType("value isn't an array".into()));
        }
        let b = self.bound();
        if !self.known_char_arr() {
            for i in 0..b {
                let t = BQNType::try_from({
                    let v = bqn_pick(self.value, i)?;
                    let t = bqn_type(v)?;
                    bqn_free(v)?;
                    t
                })
                .expect("expected known type");

                if t != BQNType::Character {
                    return Err(Error::InvalidType("value isn't a character array".into()));
                }
            }
        }
        Ok(b)
    }

    fn get_numeric_array_bounds(&self) -> Result<usize> {
        if self.bqn_type() != BQNType::Array {
            return Err(Error::InvalidType("value isn't an array".into()));
        }
        let b = self.bound();
        if !self.known_f64_arr() {
            for i in 0..b {
                let t = BQNType::try_from({
                    let v = bqn_pick(self.value, i)?;
                    let t = bqn_type(v)?;
                    bqn_free(v)?;
                    t
                })
                .expect("expected known type");

                if t != BQNType::Number {
                    return Err(Error::InvalidType("value isn't a f64 array".into()));
                }
            }
        }
        Ok(b)
    }

    // This function returns whether it's known that the array elements are of type f64
    // Returns false if *it is not known* whether the array elements are of type f64 or not
    fn known_f64_arr(&self) -> bool {
        #![allow(non_upper_case_globals)]
        matches!(
            self.direct_arr_type(),
            BQNElType_elt_f64 | BQNElType_elt_i32 | BQNElType_elt_i16 | BQNElType_elt_i8
        )
    }

    // This function returns whether it's known that the array elements are of type char
    // Returns false if *it is not known* whether the array elements are of type char or not
    fn known_char_arr(&self) -> bool {
        #![allow(non_upper_case_globals)]
        matches!(
            self.direct_arr_type(),
            BQNElType_elt_c32 | BQNElType_elt_c16 | BQNElType_elt_c8
        )
    }
}

impl fmt::Debug for BQNValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = crate::eval("•Fmt").expect("fmt");
        let formatted = fmt.call1(self).expect("fmt.call1");
        write!(
            f,
            "{}",
            formatted.to_string().expect("formatted.to_string()")
        )
    }
}

impl Clone for BQNValue {
    fn clone(&self) -> BQNValue {
        let _l = LOCK.lock();
        BQNValue {
            value: bqn_copy(self.value).unwrap(),
        }
    }
}

impl Drop for BQNValue {
    fn drop(&mut self) {
        let _l = LOCK.lock();
        bqn_free(self.value).unwrap();
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
    let key = BQNValue::new(obj)
        .to_f64()
        .expect("boundfn obj to_f64")
        .to_bits() as usize;

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
    let key = BQNValue::new(obj)
        .to_f64()
        .expect("boundfn obj to_f64")
        .to_bits() as usize;

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
/// let bqnv = eval("1+1").unwrap();
/// let bqnfn = eval("{𝕩×10}").unwrap();
/// ```
pub fn eval(bqn: &str) -> Result<BQNValue> {
    INIT.call_once(|| {
        let _l = LOCK.lock();
        bqn_init().unwrap();
    });
    let _l = LOCK.lock();
    backend_eval(bqn)
}

/// Initializes the CBQN interpreter
///
/// This is called automatically when first using the crate. This function exists to be able to
/// call the initialize function before using the library, i.e. in a background thread.
///
/// Mostly useful with WASI backend as it will take some hundred milliseconds to compile the WASI
/// module, which is done at the first use of the module.
pub fn init() {
    let _l = LOCK.lock();
    crate::INIT.call_once(|| bqn_init().unwrap());
}
