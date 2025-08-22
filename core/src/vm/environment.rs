use std::collections::HashMap;

use super::variable::Variable;

type Scope = HashMap<String, u64>;

pub struct Environment {
    current_reference: u64,
    variables: Vec<Scope>,
    references: HashMap<u64, Variable>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { 
            current_reference: 0,
            variables: vec![
                HashMap::new()
            ], 
            references: HashMap::new()
        }
    }

    fn current_scope(&self) -> Option<&Scope> {
        self.variables.last()
    }

    fn current_scope_mut(&mut self) -> Option<&mut Scope> {
        self.variables.last_mut()
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

    pub fn find_reference_mut(&mut self, name: &str) -> Option<u64> {
        for scope in self.variables.iter().rev() {
            let reference = scope.get(name);
            
            if let Some(reference) = reference {
                return Some(*reference);
            }
        }

        None
    }

    pub fn variable(&self, reference: &u64) -> Option<&Variable> {
        self.references.get(reference)
    }

    pub fn variable_mut(&mut self, reference: &u64) -> Option<&mut Variable> {
        self.references.get_mut(reference)
    }

    pub fn define_variable(&mut self, name: &str, variable: Variable) {
        let current_reference = self.current_reference;
        let current_scope = self.current_scope_mut();
        
        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                current_reference
            );

            self.references.insert(current_reference, variable);
            self.current_reference += 1;
        }
    }

    pub fn define_reference(&mut self, name: &str, reference: &u64) {
        let current_scope = self.current_scope_mut();

        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                *reference
            );
        }
    }
}