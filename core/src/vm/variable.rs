use super::value::Value;

#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    reference: Option<u64>,
    depth: usize,
} 

impl Variable {
    pub fn new(value: Value, reference: u64, depth: usize) -> Variable {
        Variable {
            value,
            reference: Some(reference),
            depth
        }
    }

    pub fn empty(value: Value, depth: usize) -> Variable {
        Variable {
            value,
            reference: None,
            depth
        }
    }

    pub fn null(depth: usize) -> Variable {
        Variable { 
            value: Value::Null, 
            reference: None,
            depth
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn reference(&self) -> &Option<u64> {
        &self.reference
    }
    pub fn set_reference(&mut self, reference: &u64) {
        self.reference = Some(*reference);
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    pub fn owned(&self) -> bool {
        if let Some(_) = self.reference {
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.reference = None;
    }
}
