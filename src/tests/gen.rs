use crate::{eval, BQNValue, BQN};

pub fn c8_str() -> BQNValue {
    BQN!(r#""hello""#)
}

pub fn c16_str() -> BQNValue {
    BQN!(r#""hello∘""#)
}

pub fn c32_str() -> BQNValue {
    BQN!(r#""hello💣""#)
}

pub fn ns() -> BQNValue {
    BQN!("{a⇐1, B⇐{a+𝕩}}")
}
