use std::collections::HashMap;

use crate::global::utils::ks_error::KsError;
use crate::global::utils::ks_result::KsResult;
use crate::vm::anchor::variable_frame::VariableFrame;
use crate::vm::anchor::variable_iter::VariableIter;

use super::anchor::reference_frame::ReferenceFrame;
use super::anchor::tree_reference::TreeReference;
use super::variable::Variable;
use super::value::Value;
use super::anchor::frame::Frame;
use super::var_info::VarInfo;

type Scope = HashMap<String, u64>;
type ScopeReference = HashMap<u64, Variable>;
type ScopeInfo = HashMap<u64, VarInfo>;

pub struct Environment {
    current_reference: u64,
    variables: Vec<Scope>,
    references: Vec<ScopeReference>,
    infos: ScopeInfo,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            current_reference: 0,
            variables: Vec::new(),
            references: Vec::new(),
            infos: HashMap::new(),
        }
    }

    pub fn variables(&self) -> &Vec<Scope> {
        &self.variables
    }

    pub fn references(&self) -> &Vec<ScopeReference> {
        &self.references
    }

    fn current_scope_mut(&mut self) -> KsResult<&mut Scope> {
        let last = self.variables.last_mut();
        if let Some(last) = last {
            Ok(last)
        } else {
            Err(KsError::runtime("Cannot access the last scope!"))
        }
    }

    fn current_scope_reference_mut(&mut self) -> KsResult<&mut ScopeReference> {
        let last = self.references.last_mut();
        if let Some(last) = last {
            Ok(last)
        } else {
            Err(KsError::runtime("Cannot access the last scope refernce!"))
        }
    }

    fn variables_depth_mut(&mut self, depth: usize) -> KsResult<&mut Scope> {
        let variables = self.variables.get_mut(depth);
        if let Some(variables) = variables {
            Ok(variables)
        } else {
            Err(KsError::runtime(
                &format!("Cannot get a scope in this depth: {}", depth)
            ))
        }
    }

    fn references_depth_mut(&mut self, depth: usize) -> KsResult<&mut ScopeReference> {
        let references = self.references.get_mut(depth);
        if let Some(references) = references {
            Ok(references)
        } else {
            Err(KsError::runtime(
                &format!("Cannot get a scope in this depth: {}", depth)
            ))
        }
    }

    pub fn find_reference(&self, name: &str) -> KsResult<u64> {
        for i in (0..self.variables.len()).rev() {
            if let Some(reference) = self.variables[i].get(name) {
                return Ok(*reference);
            }
        }

        Err(KsError::runtime(&format!("Cannot find variable {}!", name)))
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

    pub fn variable_by_depth_mut(&mut self, reference: &u64, depth: usize) -> KsResult<&mut Variable> {
        let references = self.references.get_mut(depth);

        if let Some(references) = references {
            if let Some(variable) = references.get_mut(reference) {
                Ok(variable)
            } else {
                Err(KsError::runtime(&format!("Cannot find reference {}", reference)))
            }
        } else {
            Err(KsError::runtime("Incorrect depth!"))
        }
    }

    pub fn variable_remove(&mut self, reference: &u64) -> KsResult<Variable> {
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

    pub fn define_variable(&mut self, name: &str, mut variable: Variable) -> KsResult<()> {
        let current_reference = self.current_reference;
        let current_scope = self.current_scope_mut()?;

        current_scope.insert(
            name.to_string(),
            current_reference
        );

        let current_scope_reference = self.current_scope_reference_mut()?;
        variable.set_reference(&current_reference);
        variable.add_owner();
        current_scope_reference.insert(current_reference, variable);

        self.current_reference += 1;

        Ok(())
    }

    pub fn define_variable_at_depth(&mut self, name: &str, mut variable: Variable, depth: usize) -> KsResult<()> {
        let current_reference = self.current_reference;
        let current_scope = self.variables_depth_mut(depth)?;

        current_scope.insert(
            name.to_string(),
            current_reference
        );

        let current_scope_reference = self.current_scope_reference_mut()?;
        variable.set_reference(&current_reference);
        variable.add_owner();
        current_scope_reference.insert(current_reference, variable);

        self.current_reference += 1;

        Ok(())
    }

    pub fn define_name_reference(&mut self, name: &str, reference: &u64) -> KsResult<()> {
        let current_scope = self.current_scope_mut()?;

        current_scope.insert(
            name.to_string(),
            *reference
        );

        let variable = self.variable_mut(&reference)?;
        variable.add_owner();

        Ok(())
    }

    pub fn define_name_reference_at_depth(&mut self, name: &str, reference: &u64, depth: usize) -> KsResult<()> {
        let current_scope = self.variables_depth_mut(depth)?;

        current_scope.insert(
            name.to_string(),
            *reference
        );

        let variable = self.variable_mut(&reference)?;
        variable.add_owner();

        Ok(())
    }

    pub fn define_reference(&mut self, variable: Variable) -> KsResult<u64> {
        self.current_reference += 1;
        let reference = self.current_reference - 1;
        let scope_reference = self.current_scope_reference_mut()?;

        scope_reference.insert(reference, variable);

        if let Some(variable) = scope_reference.get_mut(&(reference)) {
            variable.set_reference(&reference);
            variable.add_owner();
            Ok(reference)
        } else {
            Err(KsError::runtime("There was a problem allocating a variable!"))
        }
    }

    pub fn define_reference_at_depth(&mut self, variable: Variable, depth: usize) -> KsResult<u64> {
        self.current_reference += 1;
        let reference: u64 = self.current_reference - 1;
        let scope_reference = self.references_depth_mut(depth)?;

        scope_reference.insert(reference, variable);

        if let Some(variable) = scope_reference.get_mut(&(reference)) {
            variable.set_reference(&reference);
            variable.add_owner();
            Ok(reference)
        } else {
            Err(KsError::runtime("There was a problem allocating a variable!"))
        }
    }

    pub fn assign_to_reference(&mut self, reference: u64, mut variable: Variable) -> KsResult<()> {
        let original_variable = self.variable(&reference)?;
        let original_owners = original_variable.owners();
        
        for (depth, scope) in self.references.iter_mut().enumerate() {
            if scope.contains_key(&reference) {
                variable.set_depth(depth);
                variable.set_reference(&reference);
                variable.set_owners(original_owners);
                
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
        let current_scope = self.references_depth_mut(depth)?;
        if let Some(variable) = current_scope.get_mut(&reference) {
            variable.add_owner();
        }

        Ok(())
    }

    pub fn tree_reference<F>(&mut self, variable: Variable, mut f: F) -> KsResult<()>
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
                    Value::Module(module) => 
                        TreeReference::ModuleBranch(module, frame.index),
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
                TreeReference::ModuleBranch(module, index) => {
                    let references: Vec<&u64> = module.values().collect();
                    
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

    fn variable_iter<COLLECTION, MODULE, LEAF>(
        &mut self, 
        reference: u64, 
        mut collection_func: COLLECTION,
        mut module_func: MODULE,
        mut leaf_func: LEAF, 
    ) -> KsResult<()>
    where 
        COLLECTION: FnMut(&mut Self, VariableFrame) -> KsResult<()>,
        MODULE: FnMut(&mut Self, VariableFrame) -> KsResult<()>,
        LEAF: FnMut(&mut Self, VariableFrame) -> KsResult<()>,
    {
        let mut frames: Vec<VariableFrame> = vec![
            VariableFrame::new(reference, 0),
        ];
        
        while let Some(mut frame) = frames.pop() {
            let reference = &frame.reference;
            let variable = self.variable(reference)?;
            let next = {
                match variable.value() {
                    Value::List(references)
                    | Value::Tuple(references) =>
                        VariableIter::Collection(&references, frame.index),
                    Value::Module(module) => 
                        VariableIter::Module(&module, frame.index),
                    _ =>
                        VariableIter::Leaf,
                }
            };

            match next {
                VariableIter::Collection(collection, index) => {
                    if let Some(child_reference) = collection.get(index) {
                        frame.step();
                        frames.push(frame);
                        frames.push(VariableFrame::new(*child_reference, index))
                    } else {
                        collection_func(self, frame)?;
                    }
                },
                VariableIter::Module(module, index) => {
                    let references: Vec<&u64> = module.values().collect();

                    if let Some(child_reference) = references.get(index) {
                        frame.step();
                        frames.push(frame);
                        frames.push(VariableFrame::new(**child_reference, index))
                    } else {
                        module_func(self, frame)?;
                    }
                },
                VariableIter::Leaf => {
                    leaf_func(self, frame)?;
                },
            }
        }

        Ok(())
    }

    fn anchor_insert(&mut self, reference: u64, low_depth: usize) -> KsResult<()> {
        let mut variable = self.variable_remove(&reference)?;
        let low_scope = self.references.get_mut(low_depth);

        if let Some(low_scope) = low_scope {
            variable.set_depth(low_depth);
            low_scope.insert(reference, variable);
        }

        Ok(())
    }

    pub fn anchor(&mut self, low_depth: usize, reference: u64) -> KsResult<()> {
        self.variable_iter(
            reference, 
            |this, frame| {
                this.anchor_insert(frame.reference, low_depth)?;
                Ok(())
            }, 
            |this, frame| {
                this.anchor_insert(frame.reference, low_depth)?;
                Ok(())
            },
            |this, frame| {
                this.anchor_insert(frame.reference, low_depth)?;
                Ok(())
            },
        )?;

        Ok(())
    }

    pub fn anchor_reference(&mut self, low_depth: usize, reference: u64) -> KsResult<()> {
        self.anchor(low_depth, reference)?;
        Ok(())
    }

    fn clone_collection(&mut self, mut parent_reference: u64) -> KsResult<Variable> {
        let mut frames = vec![
            ReferenceFrame::new(parent_reference, 0),
        ];

        while let Some(mut frame) = frames.pop() {
            let next = {
                let variable = self.variable(&frame.reference)?;

                match variable.value() {
                    Value::List(references)
                    | Value::Tuple(references) =>
                        TreeReference::Branch(&references, frame.index),
                    Value::Module(module) => 
                        TreeReference::ModuleBranch(module, frame.index),
                    _ =>
                        TreeReference::Leaf,
                }
            };

            match next {
                TreeReference::Branch(references, index) => {
                    let reference = frame.reference;

                    if let Some(reference) = references.get(index) {
                        frame.step();
                        frames.push(frame);
                        frames.push(ReferenceFrame::new(*reference, 0));
                    } else {
                        let mut variable = self.variable(&reference)?.clone();
                        match variable.value_mut() {
                            Value::List(child_references)
                            | Value::Tuple(child_references) => {
                                *child_references = frame.new_references;
                            },
                            _ => {}
                        }

                        if parent_reference == reference {
                            parent_reference = self.define_reference(variable)?;
                            continue;
                        }

                        let child_reference = self.define_reference(variable)?;
                        if let Some(last_frame) = frames.last_mut() {
                            last_frame.new_references.push(child_reference);
                        }
                    }
                },
                TreeReference::ModuleBranch(module, index) => {
                    todo!()
                },
                TreeReference::Leaf => {
                    println!("here");
                    let reference = frame.reference;
                    let mut variable = self.variable(&reference)?.clone();
                    variable.clear();
                    let reference = self.define_reference(variable)?;
                    if let Some(last_frame) = frames.last_mut() {
                        last_frame.new_references.push(reference);
                    }
                }
            }
        }

        self.variable_remove(&parent_reference)
    }

    pub fn clone(&mut self, reference: u64) -> KsResult<Variable> {
        let variable = self.variable(&reference)?;

        if let Value::List(_) | Value::Tuple(_) | Value::Module(_) = variable.value() {
            let variable = variable.clone();
            let reference = self.define_reference(variable)?;
            let mut variable = self.clone_collection(reference)?;
            variable.clear();
            variable.set_depth(self.depth());

            Ok(variable)
        } else {
            let mut variable = variable.clone();
            variable.clear();
            variable.set_depth(self.depth());

            Ok(variable)
        }
    }


    /// @deprecated at least will be
    pub fn free(&mut self, reference: &u64) -> KsResult<()> {
        for i in (0..self.references.len()).rev() {
            let scope = &mut self.references[i];
            if scope.contains_key(reference) {
                if let Some(variable) = scope.remove(reference) {
                    self.tree_reference(variable, |_, frame| {
                        let mut variable = frame.variable;
                        variable.remove_owner();
                        if !variable.owned() {
                            variable.clear();
                        }
                        Ok(())
                    })?;
                }

                return Ok(())
            }
        }

        Err(KsError::runtime("No variable were found with reference!"))
    }

    // create a new function here called free_name

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
            for (name, reference) in variables {
                let variable = self.variable_mut(&reference)?;
                variable.remove_owner();

                

            }
        }

        self.references.pop();

        Ok(())
    }
}
