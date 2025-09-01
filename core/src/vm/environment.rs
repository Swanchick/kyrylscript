use std::collections::HashMap;

use crate::{global::utils::{ks_error::KsError, ks_result::KsResult}, vm::variable_stack::VariableStack};

use super::variable::Variable;

type Scope = HashMap<String, u64>;
type ScopeReference = HashMap<u64, Variable>;

pub struct Environment {
    current_reference: u64,
    variables: Vec<Scope>,
    references: Vec<ScopeReference>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { 
            current_reference: 0,
            variables: Vec::new(), 
            references: Vec::new(),
        }
    }

    fn current_scope(&self) -> Option<&Scope> {
        self.variables.last()
    }

    fn current_scope_mut(&mut self) -> Option<&mut Scope> {
        self.variables.last_mut()
    }

    fn current_scope_reference(&self) -> Option<&ScopeReference> {
        self.references.last()
    }

    fn current_scope_reference_mut(&mut self) -> Option<&mut ScopeReference> {
        self.references.last_mut()
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

    pub fn variable(&self, reference: &u64) -> KsResult<&Variable> {
        let current_scope = self.current_scope_reference();
        
        for scope in self.current_scope_reference().iter().rev() {
            if let Some(variable) = scope.get(reference) {
                return Ok(variable)
            }
        } 

        Err(KsError::runtime(&format!("Cannot find variable with reference {}", reference)))
    }

    pub fn variable_mut(&mut self, reference: &u64) -> KsResult<&mut Variable> {
        let current_scope = self.current_scope_reference_mut();

        if let Some(current_scope) = current_scope {
            if let Some(variable) = current_scope.get_mut(reference) {
                Ok(variable)
            } else {
                Err(KsError::runtime(
                    &format!("Cannot find variable with reference {}", reference)
                ))
            }
        } else {
            Err(KsError::runtime(
                "Not in the scope!"
            ))
        }
    }

    pub fn define_variable(&mut self, name: &str, variable: Variable) {
        let current_reference = self.current_reference;
        let current_scope = self.current_scope_mut();
        
        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                current_reference
            );

            if let Some(current_scope_reference) = self.current_scope_reference_mut() {
                
                current_scope_reference.insert(current_reference, variable);

                self.current_reference += 1;
            }
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

    pub fn depth(&self) -> usize {
        self.references.len()
    }

    pub fn enter(&mut self) {
        self.variables.push(HashMap::new());
        self.references.push(HashMap::new());
    }

    pub fn exit(&mut self) {
        self.variables.pop();
        self.references.pop();
    }
}