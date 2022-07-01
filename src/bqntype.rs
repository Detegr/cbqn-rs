#[derive(Debug, PartialEq)]
/// BQN type enumeration
pub enum BQNType {
    Array,
    Number,
    Character,
    Function,
    /// 1-modifier
    Mod1,
    /// 2-modifier
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
