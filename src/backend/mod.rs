use cbqn_sys as bindings;

#[derive(Debug)]
pub enum Error {
    Wasi(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub use bindings::{
    BQNElType_elt_c16, BQNElType_elt_c32, BQNElType_elt_c8, BQNElType_elt_f64, BQNElType_elt_i16,
    BQNElType_elt_i32, BQNElType_elt_i8, BQNElType_elt_unk, BQNV,
};

#[cfg(feature = "native-backend")]
mod native;

#[cfg(feature = "native-backend")]
pub use crate::backend::native::*;

#[cfg(feature = "wasi-backend")]
mod wasi;

#[cfg(feature = "wasi-backend")]
pub use crate::backend::wasi::*;
