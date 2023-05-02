use crate::BQNValue;
use cbqn_sys as bindings;

#[derive(Debug)]
pub enum Error {
    CBQN(String),
    InvalidType(String),
    NotSupported(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub use bindings::{
    BQNElType_elt_c16, BQNElType_elt_c32, BQNElType_elt_c8, BQNElType_elt_f64, BQNElType_elt_i16,
    BQNElType_elt_i32, BQNElType_elt_i8, BQNElType_elt_unk, BQNV,
};

#[cfg(feature = "native-backend")]
mod native;

#[cfg(feature = "native-backend")]
mod eval {
    use super::*;
    use once_cell::sync::OnceCell;

    static REBQN: OnceCell<BQNValue> = OnceCell::new();

    pub fn backend_eval(bqn: &str) -> Result<BQNValue> {
        let rebqn = REBQN.get_or_init(|| {
            BQNValue::new(
                bqn_eval(
                    BQNValue::from(
                        r#"r←•ReBQN{repl⇐"strict"}⋄{0‿(R𝕩)}⎊{𝕊: 1‿("Error: "∾•CurrentError@)}"#,
                    )
                    .value,
                )
                .expect("ReBQN"),
            )
        });
        let ret = rebqn.call1(&BQNValue::from(bqn))?;
        let err = bqn_pick(ret.value, 0)?;
        if err != 0 {
            let error = BQNValue::new(bqn_pick(ret.value, 1)?);
            Err(Error::CBQN(error.to_string()?))
        } else {
            Ok(BQNValue::new(bqn_pick(ret.value, 1)?))
        }
    }
}

#[cfg(feature = "native-backend")]
pub use crate::backend::{eval::backend_eval, native::*};

#[cfg(feature = "wasi-backend")]
mod wasi;

#[cfg(feature = "wasi-backend")]
pub use crate::backend::wasi::*;
