use crate::global::utils::{ks_error::KsError, ks_result::KsResult};

use super::value::Value;

#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    reference: Option<u64>,
    depth: usize,
    owners: usize,
}

impl Variable {
    pub fn new(value: Value, reference: u64, depth: usize) -> Variable {
        Variable {
            value,
            reference: Some(reference),
            depth,
            owners: 0
        }
    }

    pub fn empty(value: Value, depth: usize) -> Variable {
        Variable {
            value,
            reference: None,
            depth,
            owners: 0
        }
    }

    pub fn null(depth: usize) -> Variable {
        Variable {
            value: Value::Null,
            reference: None,
            depth,
            owners: 0
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
            Err(KsError::runtime("No error in varaible"))
        }
    }

    pub fn set_reference(&mut self, reference: &u64) {
        self.reference = Some(*reference);
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn add_owner(&mut self) {
        self.owners += 1;
    }

    pub fn remove_owner(&mut self) {
        self.owners -= 1;
    }

    pub fn clear_owners(&mut self) {
        self.owners = 0;
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    pub fn owned(&self) -> bool {
        self.owners != 0
    }

    pub fn clear(&mut self) {
        self.reference = None;
    }
}
