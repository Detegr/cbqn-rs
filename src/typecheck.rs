use cbqn_sys::*;

#[derive(Debug, PartialEq)]
pub enum BQNType {
    Array,
    Number,
    Character,
    Function,
    Mod1,
    Mod2,
    Namespace,
}
impl TryFrom<i32> for BQNType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BQNType::Array),
            1 => Ok(BQNType::Number),
            2 => Ok(BQNType::Character),
            3 => Ok(BQNType::Function),
            4 => Ok(BQNType::Mod1),
            5 => Ok(BQNType::Mod2),
            6 => Ok(BQNType::Namespace),
            _ => Err("Invalid type"),
        }
    }
}

pub const fn bqneltype_is_numeric(eltype: u32) -> bool {
    #![allow(non_upper_case_globals)]
    match eltype {
        BQNElType_elt_f64 | BQNElType_elt_i32 | BQNElType_elt_i16 | BQNElType_elt_i8 => true,
        _ => false,
    }
}

pub const fn bqneltype_is_char(eltype: u32) -> bool {
    #![allow(non_upper_case_globals)]
    match eltype {
        BQNElType_elt_c32 | BQNElType_elt_c16 | BQNElType_elt_c8 => true,
        _ => false,
    }
}

pub const fn bqneltype_is_unknown(eltype: u32) -> bool {
    eltype == BQNElType_elt_unk
}
