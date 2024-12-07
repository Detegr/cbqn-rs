#![allow(non_snake_case)]

use super::{
    bindings::{self, BQNV},
    Error, Result,
};
use crate::BQNValue;
use once_cell::sync::Lazy;
use std::{cell::UnsafeCell, io::Read, mem, num::TryFromIntError};
use wasmer::*;
use wasmer_wasix::{virtual_fs, Pipe, WasiEnv};

#[inline]
pub fn backend_eval(bqn: &str) -> Result<BQNValue> {
    Ok(BQNValue::new(bqn_eval(BQNValue::from(bqn).value)?))
}

macro_rules! impl_error(($err:ty) => {
    impl From<$err> for Error {
        fn from(_e: $err) -> Error {
            let stderr = {
                let _l = crate::LOCK.lock();
                BQNFFI.stderr_unsafe()
            };
            Error::CBQN(stderr)
        }
    }
});

impl_error!(MemoryAccessError);
impl_error!(RuntimeError);
impl_error!(TryFromIntError);

struct BqnFfi {
    free: TypedFunction<WasmPtr<u32>, ()>,
    malloc: TypedFunction<u32, WasmPtr<u32>>,

    bqn_bound: TypedFunction<BQNV, u32>,
    bqn_call1: TypedFunction<(BQNV, BQNV), BQNV>,
    bqn_call2: TypedFunction<(BQNV, BQNV, BQNV), BQNV>,
    bqn_copy: TypedFunction<BQNV, BQNV>,
    bqn_directArrType: TypedFunction<BQNV, u32>,
    bqn_free: TypedFunction<BQNV, ()>,
    bqn_getField: TypedFunction<(BQNV, BQNV), BQNV>,
    bqn_hasField: TypedFunction<(BQNV, BQNV), i32>,
    bqn_eval: TypedFunction<BQNV, BQNV>,
    bqn_init: TypedFunction<(), ()>,
    bqn_makeChar: TypedFunction<u32, BQNV>,
    bqn_makeF64: TypedFunction<f64, BQNV>,
    bqn_makeF64Vec: TypedFunction<(u32, WasmPtr<u32>), BQNV>,
    bqn_makeI32Vec: TypedFunction<(u32, WasmPtr<u32>), BQNV>,
    bqn_makeI16Vec: TypedFunction<(u32, WasmPtr<u32>), BQNV>,
    bqn_makeI8Vec: TypedFunction<(u32, WasmPtr<u32>), BQNV>,
    bqn_makeObjVec: TypedFunction<(u32, WasmPtr<u32>), BQNV>,
    bqn_makeUTF8Str: TypedFunction<(u32, WasmPtr<u32>), BQNV>,
    bqn_pick: TypedFunction<(BQNV, u32), BQNV>,
    bqn_readC32Arr: TypedFunction<(BQNV, WasmPtr<u32>), ()>,
    bqn_readChar: TypedFunction<BQNV, u32>,
    bqn_readF64: TypedFunction<BQNV, f64>,
    bqn_readF64Arr: TypedFunction<(BQNV, WasmPtr<u32>), ()>,
    bqn_readObjArr: TypedFunction<(BQNV, WasmPtr<u32>), ()>,
    bqn_type: TypedFunction<BQNV, i32>,
    bqn_rank: TypedFunction<BQNV, u32>,
    bqn_shape: TypedFunction<(BQNV, WasmPtr<u32>), ()>,

    store: UnsafeCell<Store>,
    stderr: UnsafeCell<Pipe>,
    memory: Memory,
}

impl BqnFfi {
    /// NOTE: This is highly unsafe. The caller *must* make sure there's only one &mut Store reference
    /// active at any point of execution, or otherwise Rust treats the code as undefined behavior.
    /// However, in practice, the cbqn-sys wrapper to the C shared object is also not thread-safe and
    /// the implementation in cbqn-rs crate does locking in a way that this code cannot be
    /// multi-threaded in practice.
    fn get_store_unsafe(&self) -> &mut Store {
        unsafe { self.store.get().as_mut().unwrap() }
    }

    fn stderr_unsafe(&self) -> String {
        let stderr = unsafe { self.stderr.get().as_mut().unwrap() };
        let mut ret = String::new();
        stderr.read_to_string(&mut ret).unwrap_or(0);
        ret
    }
}
/// NOTE: The type is not really Sync. The user of this library must make sure this code is not
/// going to be run in a multi-threaded context.
unsafe impl Sync for BqnFfi {}

macro_rules! wasmfn(($instance:ident, $store:ident, $name:expr) => {
    $instance.exports.get_typed_function(&$store, $name).expect($name)
});

static WASM_BYTES: &'static [u8] = include_bytes!(env!("BQN_WASM"));

static BQNFFI: Lazy<BqnFfi> = Lazy::new(|| {
    let compiler_config = Cranelift::default();
    let mut store = Store::new(compiler_config);
    let module = Module::new(&store, WASM_BYTES).expect("Create module");

    let (tx, rx) = Pipe::channel();
    let mut wasi_env = WasiEnv::builder("cbqn")
        .fs(Box::new(virtual_fs::host_fs::FileSystem))
        .stderr(Box::new(tx))
        .preopen_dir("/")
        .unwrap()
        .finalize(&mut store)
        .expect("Create WasiEnv");

    let import_object = wasi_env
        .import_object(&mut store, &module)
        .expect("Get import object");

    let instance = Instance::new(&mut store, &module, &import_object).expect("Create instance");
    wasi_env
        .initialize(&mut store, instance.clone())
        .expect("Initialize wasi_env");

    instance
        .exports
        .get_function("_initialize")
        .expect("Get _initialize function")
        .call(&mut store, &[])
        .expect("Initialize wasm module");

    BqnFfi {
        free: wasmfn!(instance, store, "free"),
        malloc: wasmfn!(instance, store, "malloc"),

        bqn_bound: wasmfn!(instance, store, "bqn_bound"),
        bqn_call1: wasmfn!(instance, store, "bqn_call1"),
        bqn_call2: wasmfn!(instance, store, "bqn_call2"),
        bqn_copy: wasmfn!(instance, store, "bqn_copy"),
        bqn_directArrType: wasmfn!(instance, store, "bqn_directArrType"),
        bqn_free: wasmfn!(instance, store, "bqn_free"),
        bqn_getField: wasmfn!(instance, store, "bqn_getField"),
        bqn_hasField: wasmfn!(instance, store, "bqn_hasField"),
        bqn_eval: wasmfn!(instance, store, "bqn_eval"),
        bqn_init: wasmfn!(instance, store, "bqn_init"),
        bqn_makeChar: wasmfn!(instance, store, "bqn_makeChar"),
        bqn_makeF64: wasmfn!(instance, store, "bqn_makeF64"),
        bqn_makeF64Vec: wasmfn!(instance, store, "bqn_makeF64Vec"),
        bqn_makeI32Vec: wasmfn!(instance, store, "bqn_makeI32Vec"),
        bqn_makeI16Vec: wasmfn!(instance, store, "bqn_makeI16Vec"),
        bqn_makeI8Vec: wasmfn!(instance, store, "bqn_makeI8Vec"),
        bqn_makeObjVec: wasmfn!(instance, store, "bqn_makeObjVec"),
        bqn_makeUTF8Str: wasmfn!(instance, store, "bqn_makeUTF8Str"),
        bqn_pick: wasmfn!(instance, store, "bqn_pick"),
        bqn_readC32Arr: wasmfn!(instance, store, "bqn_readC32Arr"),
        bqn_readChar: wasmfn!(instance, store, "bqn_readChar"),
        bqn_readF64: wasmfn!(instance, store, "bqn_readF64"),
        bqn_readF64Arr: wasmfn!(instance, store, "bqn_readF64Arr"),
        bqn_readObjArr: wasmfn!(instance, store, "bqn_readObjArr"),
        bqn_type: wasmfn!(instance, store, "bqn_type"),
        bqn_rank: wasmfn!(instance, store, "bqn_rank"),
        bqn_shape: wasmfn!(instance, store, "bqn_shape"),

        store: UnsafeCell::new(store),
        stderr: UnsafeCell::new(rx.with_blocking(false)),
        memory: instance
            .exports
            .get_memory("memory")
            .expect("Get WASM memory")
            .clone(),
    }
});

fn with_buf<BT, T, F: FnMut(&[BT], &mut Store, WasmPtr<u32>) -> Result<T>>(
    buf: &[BT],
    mut f: F,
) -> Result<T> {
    let store = BQNFFI.get_store_unsafe();
    let ptr = BQNFFI
        .malloc
        .call(store, (buf.len() * mem::size_of::<BT>()).try_into()?)?;

    let ret = f(buf, store, ptr)?;

    BQNFFI.free.call(store, ptr)?;

    Ok(ret)
}

fn with_buf_mut<BT, T, F: FnMut(&mut [BT], &mut Store, WasmPtr<u32>) -> Result<T>>(
    buf: &mut [BT],
    mut f: F,
) -> Result<T> {
    let store = BQNFFI.get_store_unsafe();
    let ptr = BQNFFI
        .malloc
        .call(store, (buf.len() * mem::size_of::<BT>()).try_into()?)?;

    let ret = f(buf, store, ptr)?;

    BQNFFI.free.call(store, ptr)?;

    Ok(ret)
}

pub fn bqn_bound(v: BQNV) -> Result<u32> {
    Ok(BQNFFI.bqn_bound.call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_call1(f: BQNV, x: BQNV) -> Result<BQNV> {
    Ok(BQNFFI.bqn_call1.call(BQNFFI.get_store_unsafe(), f, x)?)
}

pub fn bqn_call2(f: BQNV, w: BQNV, x: BQNV) -> Result<BQNV> {
    Ok(BQNFFI.bqn_call2.call(BQNFFI.get_store_unsafe(), f, w, x)?)
}

pub fn bqn_copy(v: BQNV) -> Result<BQNV> {
    Ok(BQNFFI.bqn_copy.call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_directArrType(v: BQNV) -> Result<u32> {
    Ok(BQNFFI
        .bqn_directArrType
        .call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_eval(v: BQNV) -> Result<BQNV> {
    Ok(BQNFFI.bqn_eval.call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_free(v: BQNV) -> Result<()> {
    Ok(BQNFFI.bqn_free.call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_getField(ns: BQNV, name: BQNV) -> Result<BQNV> {
    Ok(BQNFFI
        .bqn_getField
        .call(BQNFFI.get_store_unsafe(), ns, name)?)
}

pub fn bqn_hasField(ns: BQNV, name: BQNV) -> Result<bool> {
    Ok(BQNFFI
        .bqn_hasField
        .call(BQNFFI.get_store_unsafe(), ns, name)?
        != 0)
}

pub fn bqn_init() -> Result<()> {
    Ok(BQNFFI.bqn_init.call(BQNFFI.get_store_unsafe())?)
}

pub fn bqn_makeBoundFn1(_f: bindings::bqn_boundFn1, _obj: BQNV) -> Result<BQNV> {
    Err(Error::NotSupported(
        "BoundFns are not supported with WASI backend".into(),
    ))
}

pub fn bqn_makeBoundFn2(_f: bindings::bqn_boundFn2, _obj: BQNV) -> Result<BQNV> {
    Err(Error::NotSupported(
        "BoundFns are not supported with WASI backend".into(),
    ))
}

pub fn bqn_makeChar(c: u32) -> Result<BQNV> {
    Ok(BQNFFI.bqn_makeChar.call(BQNFFI.get_store_unsafe(), c)?)
}

pub fn bqn_makeF64(d: f64) -> Result<BQNV> {
    Ok(BQNFFI.bqn_makeF64.call(BQNFFI.get_store_unsafe(), d)?)
}

pub fn bqn_makeF64Vec(a: &[f64]) -> Result<BQNV> {
    with_buf(a, |buf, store, ptr| {
        let mem = BQNFFI.memory.view(store);
        let f64ptr: WasmPtr<f64> = ptr.cast();
        f64ptr.slice(&mem, buf.len().try_into()?)?.write_slice(a)?;

        Ok(BQNFFI
            .bqn_makeF64Vec
            .call(store, a.len().try_into().unwrap(), ptr)?)
    })
}

pub fn bqn_makeI32Vec(a: &[i32]) -> Result<BQNV> {
    with_buf(a, |buf, store, ptr| {
        let mem = BQNFFI.memory.view(store);
        let i32ptr: WasmPtr<i32> = ptr.cast();
        i32ptr.slice(&mem, buf.len().try_into()?)?.write_slice(a)?;

        Ok(BQNFFI
            .bqn_makeI32Vec
            .call(store, a.len().try_into().unwrap(), ptr)?)
    })
}

pub fn bqn_makeI16Vec(a: &[i16]) -> Result<BQNV> {
    with_buf(a, |buf, store, ptr| {
        let mem = BQNFFI.memory.view(store);
        let i16ptr: WasmPtr<i16> = ptr.cast();
        i16ptr.slice(&mem, buf.len().try_into()?)?.write_slice(a)?;

        Ok(BQNFFI
            .bqn_makeI16Vec
            .call(store, a.len().try_into().unwrap(), ptr)?)
    })
}

pub fn bqn_makeI8Vec(a: &[i8]) -> Result<BQNV> {
    with_buf(a, |buf, store, ptr| {
        let mem = BQNFFI.memory.view(store);
        let i8ptr: WasmPtr<i8> = ptr.cast();
        i8ptr.slice(&mem, buf.len().try_into()?)?.write_slice(a)?;

        Ok(BQNFFI
            .bqn_makeI8Vec
            .call(store, a.len().try_into().unwrap(), ptr)?)
    })
}

pub fn bqn_makeObjVec(a: &[BQNV]) -> Result<BQNV> {
    with_buf(a, |buf, store, ptr| {
        let mem = BQNFFI.memory.view(store);
        let objptr: WasmPtr<BQNV> = ptr.cast();
        objptr.slice(&mem, buf.len().try_into()?)?.write_slice(a)?;

        Ok(BQNFFI
            .bqn_makeObjVec
            .call(store, a.len().try_into().unwrap(), ptr)?)
    })
}

pub fn bqn_makeUTF8Str(s: &str) -> Result<BQNV> {
    with_buf(s.as_bytes(), |buf, store, ptr| {
        let mem = BQNFFI.memory.view(store);
        mem.write(ptr.offset().into(), buf)?;

        Ok(BQNFFI.bqn_makeUTF8Str.call(store, buf.len() as u32, ptr)?)
    })
}

pub fn bqn_pick(v: BQNV, pos: usize) -> Result<BQNV> {
    Ok(BQNFFI
        .bqn_pick
        .call(BQNFFI.get_store_unsafe(), v, pos as u32)?)
}

pub fn bqn_readC32Arr(v: BQNV, buf: &mut [u32]) -> Result<()> {
    with_buf_mut(buf, |buf, store, ptr| {
        BQNFFI.bqn_readC32Arr.call(store, v, ptr)?;
        let mem = BQNFFI.memory.view(store);
        Ok(ptr.slice(&mem, buf.len().try_into()?)?.read_slice(buf)?)
    })?;
    Ok(())
}

pub fn bqn_readChar(v: BQNV) -> Result<u32> {
    Ok(BQNFFI.bqn_readChar.call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_readF64(v: BQNV) -> Result<f64> {
    Ok(BQNFFI.bqn_readF64.call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_readF64Arr(v: BQNV, buf: &mut [f64]) -> Result<()> {
    with_buf_mut(buf, |buf, store, ptr| {
        BQNFFI.bqn_readF64Arr.call(store, v, ptr)?;
        let ptr: WasmPtr<f64> = ptr.cast();
        let mem = BQNFFI.memory.view(store);
        ptr.slice(&mem, buf.len().try_into()?)?.read_slice(buf)?;
        Ok(())
    })?;
    Ok(())
}

pub fn bqn_readObjArr(v: BQNV, buf: &mut [BQNV]) -> Result<()> {
    with_buf_mut(buf, |buf, store, ptr| {
        BQNFFI.bqn_readObjArr.call(store, v, ptr)?;
        let ptr: WasmPtr<BQNV> = ptr.cast();
        let mem = BQNFFI.memory.view(store);
        Ok(ptr.slice(&mem, buf.len().try_into()?)?.read_slice(buf)?)
    })?;
    Ok(())
}

pub fn bqn_type(v: BQNV) -> Result<i32> {
    Ok(BQNFFI.bqn_type.call(BQNFFI.get_store_unsafe(), v)?)
}

pub fn bqn_rank(v: BQNV) -> Result<usize> {
    Ok(BQNFFI.bqn_rank.call(BQNFFI.get_store_unsafe(), v)? as usize)
}

pub fn bqn_shape(v: BQNV, buf: &mut [usize]) -> Result<()> {
    // In 32-bit WASI, usize is u32 so we need convert the values back and forth
    let mut shape: Vec<u32> = Vec::with_capacity(buf.len());
    unsafe {
        shape.set_len(buf.len());
    }

    with_buf_mut(&mut shape, |buf, store, ptr| {
        BQNFFI.bqn_shape.call(store, v, ptr)?;
        let ptr: WasmPtr<u32> = ptr.cast();
        let mem = BQNFFI.memory.view(store);
        Ok(ptr.slice(&mem, buf.len().try_into()?)?.read_slice(buf)?)
    })?;

    for i in 0..buf.len() {
        buf[i] = shape[i] as usize;
    }

    Ok(())
}
