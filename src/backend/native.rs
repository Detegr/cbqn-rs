#![allow(non_snake_case)]

use super::{
    bindings::{self, BQNV},
    Result,
};

pub fn bqn_bound(v: BQNV) -> Result<bindings::size_t> {
    Ok(unsafe { bindings::bqn_bound(v) })
}

pub fn bqn_call1(f: BQNV, x: BQNV) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_call1(f, x) })
}

pub fn bqn_call2(f: BQNV, w: BQNV, x: BQNV) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_call2(f, w, x) })
}

pub fn bqn_copy(v: BQNV) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_copy(v) })
}

pub fn bqn_directArrType(v: BQNV) -> Result<u32> {
    Ok(unsafe { bindings::bqn_directArrType(v) })
}

pub fn bqn_eval(v: BQNV) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_eval(v) })
}

pub fn bqn_free(v: BQNV) -> Result<()> {
    Ok(unsafe { bindings::bqn_free(v) })
}

pub fn bqn_getField(ns: BQNV, name: BQNV) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_getField(ns, name) })
}

pub fn bqn_hasField(ns: BQNV, name: BQNV) -> Result<bool> {
    Ok(unsafe { bindings::bqn_hasField(ns, name) })
}

pub fn bqn_init() -> Result<()> {
    Ok(unsafe { bindings::bqn_init() })
}

pub fn bqn_makeBoundFn1(f: bindings::bqn_boundFn1, obj: BQNV) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeBoundFn1(f, obj) })
}

pub fn bqn_makeBoundFn2(f: bindings::bqn_boundFn2, obj: BQNV) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeBoundFn2(f, obj) })
}

pub fn bqn_makeChar(c: u32) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeChar(c) })
}

pub fn bqn_makeF64(d: f64) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeF64(d) })
}

pub fn bqn_makeF64Vec(a: &[f64]) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeF64Vec(a.len().try_into().unwrap(), a.as_ptr()) })
}

pub fn bqn_makeI32Vec(a: &[i32]) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeI32Vec(a.len().try_into().unwrap(), a.as_ptr()) })
}

pub fn bqn_makeI16Vec(a: &[i16]) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeI16Vec(a.len().try_into().unwrap(), a.as_ptr()) })
}

pub fn bqn_makeI8Vec(a: &[i8]) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeI8Vec(a.len().try_into().unwrap(), a.as_ptr()) })
}

pub fn bqn_makeObjVec(a: &[BQNV]) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeObjVec(a.len().try_into().unwrap(), a.as_ptr()) })
}

pub fn bqn_makeUTF8Str(s: &str) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_makeUTF8Str(s.len().try_into().unwrap(), s.as_ptr() as *const i8) })
}

pub fn bqn_pick(v: BQNV, pos: bindings::size_t) -> Result<BQNV> {
    Ok(unsafe { bindings::bqn_pick(v, pos) })
}

pub fn bqn_readC32Arr(v: BQNV, buf: &mut [u32]) -> Result<()> {
    Ok(unsafe { bindings::bqn_readC32Arr(v, buf.as_mut_ptr()) })
}

pub fn bqn_readChar(v: BQNV) -> Result<u32> {
    Ok(unsafe { bindings::bqn_readChar(v) })
}

pub fn bqn_readF64(v: BQNV) -> Result<f64> {
    Ok(unsafe { bindings::bqn_readF64(v) })
}

pub fn bqn_readF64Arr(v: BQNV, buf: &mut [f64]) -> Result<()> {
    Ok(unsafe { bindings::bqn_readF64Arr(v, buf.as_mut_ptr()) })
}

pub fn bqn_readObjArr(v: BQNV, buf: &mut [BQNV]) -> Result<()> {
    Ok(unsafe { bindings::bqn_readObjArr(v, buf.as_mut_ptr()) })
}

pub fn bqn_type(v: BQNV) -> Result<i32> {
    Ok(unsafe { bindings::bqn_type(v) })
}
