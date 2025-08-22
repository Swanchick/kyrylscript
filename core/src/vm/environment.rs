use std::sync::atomic::AtomicU64;
use std::collections::HashMap;

use super::variable::Variable;

type Scope = HashMap<String, u64>;

static GLOBAL_REFERENCE_COUNT: AtomicU64 = AtomicU64::new(0);

pub struct Environment {
    variables: Vec<Scope>,
    references: HashMap<u64, Variable>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { 
            variables: Vec::new(), 
            references: HashMap::new()
        }
    }

    pub fn find_reference(&self, name: &str) -> Option<u64> {
        for scope in self.variables.iter().rev() {
            let reference = scope.get(name);
            
            if let Some(reference) = reference {
                return Some(*reference);
            }
        }

        None
    }
}