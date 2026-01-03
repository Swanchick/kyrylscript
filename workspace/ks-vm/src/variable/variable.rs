use ks_global::utils::{ks_error::KsError, ks_result::KsResult};

use crate::environment::Reference;

use super::value::Value;

#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    reference: Option<Reference>,
}

impl Variable {
    pub fn new(value: Value, reference: Reference) -> Variable {
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

    pub fn reference(&self) -> KsResult<Reference> {
        if let Some(reference) = self.reference {
            Ok(reference)
        } else {
            Err(KsError::runtime("No reference in variable!"))
        }
    }

    pub fn set_reference(&mut self, reference: &Reference) {
        self.reference = Some(*reference);
    }

    pub fn clear(&mut self) {
        self.reference = None;
    }
}
