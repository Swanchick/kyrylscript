use std::collections::HashMap;
use std::thread::current;

use crate::global::utils::ks_error::KsError;
use crate::global::utils::ks_result::KsResult;
use crate::vm::variable;
use crate::vm::variable_stack::VariableStack;

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

    pub fn variables(&self) -> &Vec<Scope> {
        &self.variables
    }

    pub fn references(&self) -> &Vec<ScopeReference> {
        &self.references
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
        for scope in self.references.iter().rev() {
            if let Some(variable) = scope.get(reference) {
                return Ok(variable)
            }
        } 

        Err(KsError::runtime(
            &format!("Cannot find variable with reference {}", reference)
        ))
    }

    pub fn variable_mut(&mut self, reference: &u64) -> KsResult<&mut Variable> {
        let scopes = &mut self.references;
        
        for scope in scopes.iter_mut().rev() {
            if let Some(variable) = scope.get_mut(reference) {
                return Ok(variable)
            }
        } 

        Err(KsError::runtime(
            &format!("Cannot find variable with reference {}", reference)
        ))
    }

    pub fn define_variable(&mut self, name: &str, mut variable: Variable) {
        let current_reference = self.current_reference;
        let current_scope = self.current_scope_mut();
        
        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                current_reference
            );

            if let Some(current_scope_reference) = self.current_scope_reference_mut() {
                variable.set_reference(&current_reference);
                current_scope_reference.insert(current_reference, variable);

                self.current_reference += 1;
            }
        }
    }

    pub fn define_name_reference(&mut self, name: &str, reference: &u64) {
        let current_scope = self.current_scope_mut();

        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                *reference
            );
        }
    }

    fn define_reference(&mut self, variable: Variable) -> KsResult<&mut Variable> {
        self.current_reference += 1;
        let reference = self.current_reference - 1;
        let scope_reference = self.current_scope_reference_mut();

        if let Some(scope_reference) = scope_reference {
            scope_reference.insert(reference, variable);

            if let Some(variable) = scope_reference.get_mut(&(reference)) {
                variable.set_reference(&reference);
                Ok(variable)
            } else {
                Err(KsError::runtime("There was a problem allocating a variable!"))
            }
        } else {
            Err(KsError::runtime("No reference scope!"))
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