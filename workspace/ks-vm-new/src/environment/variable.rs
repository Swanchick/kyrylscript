use ks_global::utils::{ks_error::KsError, ks_result::KsResult};

use crate::types::{CollectionId, Owners, Pointer};

pub const NULL_TYPE: u8 = 0;
pub const INT_TYPE: u8 = 1;
pub const FLOAT_TYPE: u8 = 2;
pub const STRING_TYPE: u8 = 3;
pub const BOOLEAN_TYPE: u8 = 4;
pub const STACK_TYPE: u8 = 5;
pub const FUNCTION_TYPE: u8 = 6;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub value: u64,
    pub owners: Owners,
    pub value_type: u8,
}

impl From<i64> for Variable {
    fn from(value: i64) -> Self {
        Self::new(INT_TYPE, value as u64)
    }
}

impl From<f64> for Variable {
    fn from(value: f64) -> Self {
        Self::new(FLOAT_TYPE, value.to_bits())
    }
}

impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Self::new(BOOLEAN_TYPE, if value { 1 } else { 0 })
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

    pub fn with_owners(mut self, owners: Owners) -> Self {
        self.owners = owners;
        self
    }

    pub fn null() -> Self {
        Self::new(NULL_TYPE, 0)
    }

    pub fn string(collection_id: CollectionId) -> Self {
        Self::new(STRING_TYPE, collection_id)
    }

    pub fn collection(collection_id: CollectionId) -> Self {
        Self::new(STACK_TYPE, collection_id)
    }

    pub fn function(pointer: Pointer) -> Self {
        Self::new(FUNCTION_TYPE, pointer as u64)
    }

    pub fn as_boolean(&self) -> bool {
        self.value == 1
    }

    pub fn is_primitive(&self) -> bool {
        matches!(
            self.value_type,
            INT_TYPE | FLOAT_TYPE | NULL_TYPE | BOOLEAN_TYPE | FUNCTION_TYPE
        )
    }

    pub fn is_string(&self) -> bool {
        self.value_type == STRING_TYPE
    }

    pub fn is_stack(&self) -> bool {
        self.value_type == STACK_TYPE
    }

    pub fn as_f64(&self) -> KsResult<f64> {
        match self.value_type {
            INT_TYPE => Ok(self.value as i64 as f64),
            FLOAT_TYPE => Ok(f64::from_bits(self.value)),
            _ => Err(KsError::runtime("Cannot convert to float")),
        }
    }
}
