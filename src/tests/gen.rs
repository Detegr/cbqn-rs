use crate::{eval, BQNValue, BQN};

pub fn c8_str() -> BQNValue {
    BQN!(r#""hello""#).unwrap()
}

pub fn c16_str() -> BQNValue {
    BQN!(r#""hello∘""#).unwrap()
}

pub fn c32_str() -> BQNValue {
    BQN!(r#""hello💣""#).unwrap()
}

pub fn ns() -> BQNValue {
    BQN!("{a⇐1, B⇐{a+𝕩}}").unwrap()
}
