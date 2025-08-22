use super::value::Value;


pub struct Variable {
    value: Value,
    reference: Option<u64>,
} 

impl Variable {
    pub fn new(value: Value, reference: u64) -> Variable {
        Variable {
            value,
            reference: Some(reference)
        }
    }

    pub fn empty(value: Value ) -> Variable {
        Variable {
            value,
            reference: None
        }
    }

    pub fn null() -> Variable {
        Variable { 
            value: Value::Null, 
            reference: None
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn reference(&self) -> &Option<u64> {
        &self.reference
    }

    pub fn owned(&self) -> bool {
        if let Some(_) = self.reference {
            true
        } else {
            false
        }
    }
}
