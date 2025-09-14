use std::collections::HashMap;

use crate::global::utils::ks_error::KsError;
use crate::global::utils::ks_result::KsResult;
use crate::vm::anchor::tree_reference::TreeReference;

use super::variable::Variable;
use super::value::Value;
use super::anchor::frame::Frame;

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

    fn current_scope_mut(&mut self) -> Option<&mut Scope> {
        self.variables.last_mut()
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

    fn variable_remove(&mut self, reference: &u64) -> KsResult<Variable> {
        for scope in self.references.iter_mut().rev() {
            if let Some(variable) = scope.remove(reference) {
                return Ok(variable)
            }
        } 

        Err(KsError::runtime(
            &format!("Cannot find variable with reference {}", reference)
        ))
    }

    pub fn reference(&self, name: &str) -> KsResult<u64> {
        for scope in &self.variables {
            if let Some(reference) = scope.get(name) {
                return Ok(*reference)
            }
        }

        Err(KsError::runtime(
            &format!("No reference has been found by name {}", name)
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
                variable.add_owner();
                current_scope_reference.insert(current_reference, variable);

                self.current_reference += 1;
            }
        }
    }

    pub fn define_variable_at_depth(&mut self, name: &str, mut variable: Variable, depth: usize) {
        let current_reference = self.current_reference;
        let current_scope = self.variables.get_mut(depth);
        
        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                current_reference
            );

            if let Some(current_scope_reference) = self.current_scope_reference_mut() {
                variable.set_reference(&current_reference);
                variable.add_owner();
                current_scope_reference.insert(current_reference, variable);

                self.current_reference += 1;
            }
        }
    }

    pub fn define_name_reference(&mut self, name: &str, reference: &u64) -> KsResult<()> {
        let current_scope = self.current_scope_mut();

        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                *reference
            );
        }

        let variable = self.variable_mut(&reference)?;
        variable.add_owner();

        Ok(())
    }

    pub fn define_name_reference_at_depth(&mut self, name: &str, reference: &u64, depth: usize) -> KsResult<()> {
        let current_scope = self.variables.get_mut(depth);

        if let Some(current_scope) = current_scope {
            current_scope.insert(
                name.to_string(), 
                *reference
            );
        }

        let variable = self.variable_mut(&reference)?;
        variable.add_owner();

        Ok(())
    }

    pub fn define_reference(&mut self, variable: Variable) -> KsResult<u64> {
        self.current_reference += 1;
        let reference = self.current_reference - 1;
        let scope_reference = self.current_scope_reference_mut();

        if let Some(scope_reference) = scope_reference {
            scope_reference.insert(reference, variable);

            if let Some(variable) = scope_reference.get_mut(&(reference)) {
                variable.set_reference(&reference);
                variable.add_owner();
                Ok(reference)
            } else {
                Err(KsError::runtime("There was a problem allocating a variable!"))
            }
        } else {
            Err(KsError::runtime("No reference scope!"))
        }
    }

    pub fn define_reference_at_depth(&mut self, variable: Variable, depth: usize) -> KsResult<u64> {
        self.current_reference += 1;
        let reference: u64 = self.current_reference - 1;
        let scope_reference = self.references.get_mut(depth);

        if let Some(scope_reference) = scope_reference {
            scope_reference.insert(reference, variable);

            if let Some(variable) = scope_reference.get_mut(&(reference)) {
                variable.set_reference(&reference);
                variable.add_owner();
                Ok(reference)
            } else {
                Err(KsError::runtime("There was a problem allocating a variable!"))
            }
        } else {
            Err(KsError::runtime("No reference scope!"))
        }
    }

    pub fn assign_to_reference(&mut self, reference: u64, mut variable: Variable) -> KsResult<()> {
        for (depth, scope) in self.references.iter_mut().enumerate() {
            if scope.contains_key(&reference) {
                variable.set_depth(depth);
                variable.set_reference(&reference);
                variable.add_owner();
                scope.insert(reference, variable);
                break;
            }
        }

        Ok(())
    }

    pub fn assign_to_name(&mut self, name: &str, reference: &u64) -> KsResult<()> {
        for scope in self.variables.iter_mut() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), *reference);
            }
        }

        Ok(())
    }

    pub fn add_variable_owner(&mut self, reference: u64, depth: usize) -> KsResult<()> {
        if let Some(current_scope) = self.references.get_mut(depth) {
            if let Some(variable) = current_scope.get_mut(&reference) {
                variable.add_owner();
            }

            Ok(())
        } else {
            Err(KsError::runtime("No scope were found!"))
        }
    }

    fn tree_reference<F>(&mut self, variable: Variable, mut f: F) -> KsResult<()> 
    where F: FnMut(&mut Self, Frame) -> KsResult<()>
    {
        let mut frames: Vec<Frame> = vec![
            Frame::new(variable, 0)
        ];

        while let Some(mut frame) = frames.pop() {
            let next = {
                match frame.variable.value() {
                    Value::List(references) 
                    | Value::Tuple(references) => 
                        TreeReference::Branch(&references, frame.index),
                    _ => 
                        TreeReference::Leaf,
                }
            };

            match next {
                TreeReference::Branch(references, index) => {
                    if let Some(reference) = references.get(index) {
                        let child = self.variable_remove(reference)?;
                        frame.step();
                        frames.push(frame);
                        frames.push(Frame::new(child, 0))
                    } else {
                        f(self, frame)?;
                    }
                },
                TreeReference::Leaf =>
                    f(self, frame)?,
            }
        }

        Ok(())
    }

    fn anchor_insert(&mut self, variable: Variable, low_depth: usize) -> KsResult<()> {
        let low_scope = self.references.get_mut(low_depth);
        
        if let (Some(reference), Some(low_scope)) = (variable.reference(), low_scope) {
            low_scope.insert(*reference, variable);
        } 
        
        Ok(())
    }

    fn anchor_references(&mut self, variable: Variable, low_depth: usize) -> KsResult<()> {
        self.tree_reference(variable, |this, frame| {
            this.anchor_insert(frame.variable, low_depth)
        })?;

        Ok(())
    }

    pub fn anchor(&mut self, low_depth: usize, reference: u64) -> KsResult<()> {        
        let variable = self.variable_remove(&reference)?;
        self.anchor_references(variable, low_depth)?;

        Ok(())
    }

    pub fn free(&mut self, reference: &u64) -> KsResult<()> {
        for scope in self.references.iter_mut() {
            if scope.contains_key(reference) {
                if let Some(variable) = scope.get_mut(reference) {
                    variable.remove_owner();
                    
                    if !variable.owned() {
                        variable.clear();
                        scope.remove(reference);
                    }
                }

                return Ok(())
            }
        }

        Err(KsError::runtime("No variable were found with reference!"))
    }

    pub fn debug(&self) {
        println!("Variables =================");
        for (depth, scope) in self.variables.iter().enumerate() {
            println!("Depth: {}", depth);
            
            for (name, reference) in scope {
                println!("{} -> {}", name, reference);
            }
        }
        println!("References ================");
        for (depth, scope) in self.references.iter().enumerate() {
            println!("Depth: {}", depth);
            
            for (reference, variable) in scope {
                println!("{}: {:?}", reference, variable);
            }
        }
        println!("===========================");
    }

    pub fn depth(&self) -> usize {
        (self.references.len() as i32 - 1).max(0) as usize
    }

    pub fn enter(&mut self) {
        self.variables.push(HashMap::new());
        self.references.push(HashMap::new());
    }

    pub fn exit(&mut self) -> KsResult<()> {
        let variables = self.variables.pop();
        if let Some(variables) = variables {
            for (_, reference) in variables {
                let variable = self.variable_mut(&reference)?;
                variable.remove_owner();
            }
        } 

        self.references.pop();

        Ok(())
    }
}