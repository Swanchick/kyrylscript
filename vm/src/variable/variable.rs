use global::utils::{ks_error::KsError, ks_result::KsResult};

use super::value::Value;

#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    reference: Option<u64>,
}

impl Variable {
    pub fn new(value: Value, reference: u64) -> Variable {
        Variable {
            value,
            reference: Some(reference),
        }
    }

    pub fn empty(value: Value) -> Variable {
        Variable {
            value,
            reference: None,
        }
    }

    pub fn null() -> Variable {
        Variable {
            value: Value::Null,
            reference: None,
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn reference(&self) -> KsResult<u64> {
        if let Some(reference) = self.reference {
            Ok(reference)
        } else {
            Err(KsError::runtime("No reference in variable!"))
        }
    }

    pub fn set_reference(&mut self, reference: &u64) {
        self.reference = Some(*reference);
    }

    pub fn clear(&mut self) {
        self.reference = None;
    }
}
