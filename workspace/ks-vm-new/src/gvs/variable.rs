pub const NULL_TYPE: u8 = 0;
pub const INT_TYPE: u8 = 1;
pub const FLOAT_TYPE: u8 = 2;
pub const STRING_TYPE: u8 = 3;
pub const BOOLEAN_TYPE: u8 = 4;

#[derive(Debug, PartialEq)]
pub struct Variable {
    owners: u32,
    value_type: u8,
    value: u64,
}

impl From<i64> for Variable {
    fn from(value: i64) -> Self {
        Self::new(INT_TYPE, value as u64)
    }
}

impl From<f64> for Variable {
    fn from(value: f64) -> Self {
        Self::new(FLOAT_TYPE, value as u64)
    }
}

impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Self::new(BOOLEAN_TYPE, value as u64)
    }
}

impl Variable {
    pub fn new(value_type: u8, value: u64) -> Self {
        Self {
            owners: 0,
            value_type,
            value,
        }
    }

    pub fn null() -> Self {
        Self {
            owners: 0,
            value_type: NULL_TYPE,
            value: 0,
        }
    }
}
