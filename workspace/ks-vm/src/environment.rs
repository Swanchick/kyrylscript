use std::collections::HashMap;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::variable::value::Value;
use crate::variable::var_info::VarInfo;
use crate::variable::variable::Variable;
use crate::variable::variable_frame::VariableFrame;
use crate::variable::variable_iter::VariableIter;

use super::anchor::reference_frame::ReferenceFrame;
use super::anchor::tree_reference::TreeReference;

pub type Reference = usize;
pub type Depth = usize;
pub type Owners = usize;

type Scope = HashMap<String, Reference>;
type ScopeReference = HashMap<Reference, Variable>;
type ScopeInfo = HashMap<Reference, VarInfo>;

pub struct Environment {
    current_reference: Reference,
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

    pub fn variables(&self) -> &[Scope] {
        &self.variables
    }

    pub fn references(&self) -> &[ScopeReference] {
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

    fn references_depth(&self, depth: Depth) -> KsResult<&ScopeReference> {
        let references = self.references.get(depth);
        if let Some(references) = references {
            Ok(references)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get a scope in this depth: {}",
                depth
            )))
        }
    }

    fn references_depth_mut(&mut self, depth: Depth) -> KsResult<&mut ScopeReference> {
        let references = self.references.get_mut(depth);
        if let Some(references) = references {
            Ok(references)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get a scope in this depth: {}",
                depth
            )))
        }
    }

    fn variables_depth_mut(&mut self, depth: Depth) -> KsResult<&mut Scope> {
        let variables = self.variables.get_mut(depth);
        if let Some(variables) = variables {
            Ok(variables)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get a scope in this depth: {}",
                depth
            )))
        }
    }

    pub fn find_reference(&self, name: &str) -> KsResult<Reference> {
        for i in (0..self.variables.len()).rev() {
            if let Some(reference) = self.variables[i].get(name) {
                return Ok(*reference);
            }
        }

        Err(KsError::runtime(&format!("Cannot find variable {}!", name)))
    }

    pub fn variable(&self, reference: &Reference) -> KsResult<&Variable> {
        let info = self.info(reference)?;
        let scope = self.references_depth(*info.depth())?;

        if let Some(variable) = scope.get(reference) {
            Ok(variable)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot find variable with reference {}",
                reference
            )))
        }
    }

    pub fn variable_mut(&mut self, reference: &Reference) -> KsResult<&mut Variable> {
        let info = self.info(reference)?;
        let scope = self.references_depth_mut(*info.depth())?;

        if let Some(variable) = scope.get_mut(reference) {
            Ok(variable)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot find variable with reference {}",
                reference
            )))
        }
    }

    pub fn variable_remove(&mut self, reference: &Reference) -> KsResult<Variable> {
        let depth = {
            let info = self.info(reference)?;
            *info.depth()
        };

        let scope = self.references_depth_mut(depth)?;

        if let Some(variable) = scope.remove(reference) {
            Ok(variable)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot find variable with reference {}",
                reference
            )))
        }
    }

    pub fn reference(&self, name: &str) -> KsResult<Reference> {
        for scope in &self.variables {
            if let Some(reference) = scope.get(name) {
                return Ok(*reference);
            }
        }

        Err(KsError::runtime(&format!(
            "No reference has been found by name {}",
            name
        )))
    }

    pub fn info(&self, reference: &Reference) -> KsResult<&VarInfo> {
        let var_info = self.infos.get(reference);
        if let Some(var_info) = var_info {
            Ok(var_info)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot variable info by that reference {}",
                reference
            )))
        }
    }

    pub fn info_mut(&mut self, reference: &Reference) -> KsResult<&mut VarInfo> {
        let var_info = self.infos.get_mut(reference);
        if let Some(var_info) = var_info {
            Ok(var_info)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot variable info by that reference {}",
                reference
            )))
        }
    }

    fn owned(&self, reference: &Reference) -> KsResult<bool> {
        let info = self.info(reference)?;
        Ok(info.owned())
    }

    pub fn add_owner(&mut self, reference: &Reference) -> KsResult<()> {
        self.variable_iter(
            *reference,
            |this, frame| {
                let info = this.info_mut(&frame.reference)?;
                info.add_owner();
                Ok(())
            },
            |this, frame| {
                let info = this.info_mut(&frame.reference)?;
                info.add_owner();
                Ok(())
            },
            |this, frame| {
                let info = this.info_mut(&frame.reference)?;
                info.add_owner();
                Ok(())
            },
        )
    }

    pub fn remove_owner(&mut self, reference: &Reference) -> KsResult<()> {
        let info = self.info_mut(reference)?;
        info.remove_owner();

        Ok(())
    }

    pub fn set_owners(&mut self, reference: &Reference, owners: Owners) -> KsResult<()> {
        let info = self.info_mut(reference)?;
        info.set_owners(owners);

        Ok(())
    }

    pub fn set_depth(&mut self, reference: &Reference, depth: Depth) -> KsResult<()> {
        let info = self.info_mut(reference)?;
        info.set_depth(depth);

        Ok(())
    }

    pub fn define_variable(&mut self, name: &str, mut variable: Variable) -> KsResult<Reference> {
        let current_reference = self.current_reference;

        let current_scope = self.current_scope_mut()?;
        current_scope.insert(name.to_string(), current_reference);

        let mut var_info = VarInfo::new(current_reference, self.depth());
        var_info.add_owner();
        self.infos.insert(current_reference, var_info);

        let current_scope_reference = self.current_scope_reference_mut()?;
        variable.set_reference(&current_reference);

        current_scope_reference.insert(current_reference, variable);

        self.current_reference += 1;

        Ok(self.current_reference - 1)
    }

    pub fn define_variable_at_depth(
        &mut self,
        name: &str,
        mut variable: Variable,
        depth: Depth,
    ) -> KsResult<()> {
        let current_reference = self.current_reference;
        let current_scope = self.variables_depth_mut(depth)?;

        current_scope.insert(name.to_string(), current_reference);

        let mut var_info = VarInfo::new(current_reference, depth);
        var_info.add_owner();
        self.infos.insert(current_reference, var_info);

        let current_scope_reference = self.current_scope_reference_mut()?;
        variable.set_reference(&current_reference);
        current_scope_reference.insert(current_reference, variable);

        self.current_reference += 1;

        Ok(())
    }

    pub fn define_name_reference(&mut self, name: &str, reference: &Reference) -> KsResult<()> {
        self.add_owner(reference)?;

        let current_scope = self.current_scope_mut()?;
        current_scope.insert(name.to_string(), *reference);

        Ok(())
    }

    pub fn define_name_reference_at_depth(
        &mut self,
        name: &str,
        reference: &Reference,
        depth: Depth,
    ) -> KsResult<()> {
        self.add_owner(reference)?;

        let current_scope = self.variables_depth_mut(depth)?;
        current_scope.insert(name.to_string(), *reference);

        Ok(())
    }

    pub fn define_reference(&mut self, mut variable: Variable) -> KsResult<Reference> {
        self.current_reference += 1;
        let reference = self.current_reference - 1;

        let mut var_info = VarInfo::new(reference, self.depth());
        var_info.add_owner();
        self.infos.insert(reference, var_info);

        let scope_reference = self.current_scope_reference_mut()?;
        variable.set_reference(&reference);
        scope_reference.insert(reference, variable);

        Ok(reference)
    }

    pub fn define_reference_at_depth(
        &mut self,
        mut variable: Variable,
        depth: Depth,
    ) -> KsResult<Reference> {
        self.current_reference += 1;
        let reference = self.current_reference - 1;

        let mut var_info = VarInfo::new(reference, depth);
        var_info.add_owner();
        self.infos.insert(reference, var_info);

        let scope_reference = self.references_depth_mut(depth)?;
        variable.set_reference(&reference);
        scope_reference.insert(reference, variable);

        Ok(reference)
    }

    pub fn assign_to_reference(
        &mut self,
        reference: Reference,
        mut variable: Variable,
    ) -> KsResult<()> {
        let info = self.info(&reference)?;
        let scope_reference = self.references_depth_mut(*info.depth())?;
        variable.set_reference(&reference);

        scope_reference.insert(reference, variable);

        Ok(())
    }

    pub fn assign_to_name(&mut self, name: &str, reference: &Reference) -> KsResult<()> {
        for scope in self.variables.iter_mut() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), *reference);
            }
        }

        Ok(())
    }

    fn variable_iter<COLLECTION, MODULE, LEAF>(
        &mut self,
        reference: Reference,
        mut collection_func: COLLECTION,
        mut module_func: MODULE,
        mut leaf_func: LEAF,
    ) -> KsResult<()>
    where
        COLLECTION: FnMut(&mut Self, VariableFrame) -> KsResult<()>,
        MODULE: FnMut(&mut Self, VariableFrame) -> KsResult<()>,
        LEAF: FnMut(&mut Self, VariableFrame) -> KsResult<()>,
    {
        let mut frames: Vec<VariableFrame> = vec![VariableFrame::new(reference, 0)];

        while let Some(mut frame) = frames.pop() {
            let reference = &frame.reference;
            let variable = self.variable(reference)?;
            let next = {
                match variable.value() {
                    Value::List(references) | Value::Tuple(references) => {
                        VariableIter::Collection(&references, frame.index)
                    }
                    Value::Module(module) => VariableIter::Module(&module, frame.index),
                    _ => VariableIter::Leaf,
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
                }
                VariableIter::Module(module, index) => {
                    let references: Vec<&Reference> = module.values().collect();

                    if let Some(child_reference) = references.get(index) {
                        frame.step();
                        frames.push(frame);
                        frames.push(VariableFrame::new(**child_reference, index))
                    } else {
                        module_func(self, frame)?;
                    }
                }
                VariableIter::Leaf => {
                    leaf_func(self, frame)?;
                }
            }
        }

        Ok(())
    }

    fn anchor_insert(&mut self, reference: Reference, low_depth: Depth) -> KsResult<()> {
        let variable = self.variable_remove(&reference)?;
        let low_scope = self.references_depth_mut(low_depth)?;
        low_scope.insert(reference, variable);

        self.set_depth(&reference, low_depth)?;

        Ok(())
    }

    pub fn anchor(&mut self, low_depth: Depth, reference: Reference) -> KsResult<()> {
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

    pub fn anchor_reference(&mut self, low_depth: Depth, reference: Reference) -> KsResult<()> {
        self.anchor(low_depth, reference)?;
        Ok(())
    }

    fn clone_collection(&mut self, mut parent_reference: Reference) -> KsResult<Variable> {
        let mut frames = vec![ReferenceFrame::new(parent_reference, 0)];

        while let Some(mut frame) = frames.pop() {
            let next = {
                let variable = self.variable(&frame.reference)?;

                match variable.value() {
                    Value::List(references) | Value::Tuple(references) => {
                        TreeReference::Branch(&references, frame.index)
                    }
                    Value::Module(module) => TreeReference::ModuleBranch(module, frame.index),
                    _ => TreeReference::Leaf,
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
                        let variable = self.variable(&reference)?;
                        let mut variable = variable.clone();
                        match variable.value_mut() {
                            Value::List(child_references) | Value::Tuple(child_references) => {
                                *child_references = frame.new_references
                            }
                            _ => unreachable!(),
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
                }
                TreeReference::ModuleBranch(module, index) => {
                    let reference = frame.reference;
                    let references: Vec<&Reference> = module.values().collect();
                    if let Some(reference) = references.get(index) {
                        frame.step();
                        frames.push(frame);
                        frames.push(ReferenceFrame::new(**reference, 0));
                    } else {
                        let variable = self.variable(&frame.reference)?;
                        let mut variable = variable.clone();

                        match variable.value_mut() {
                            Value::Module(module) => {
                                let mut new_module: HashMap<String, Reference> = HashMap::new();
                                let keys: Vec<&String> = module.keys().collect();

                                for (key, value) in keys.iter().zip(frame.new_references) {
                                    new_module.insert(key.to_string(), value);
                                }

                                *module = new_module;
                            }
                            _ => unreachable!(),
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
                }
                TreeReference::Leaf => {
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

    pub fn clone(&mut self, reference: Reference) -> KsResult<Variable> {
        let variable = self.variable(&reference)?;

        if let Value::List(_) | Value::Tuple(_) | Value::Module(_) = variable.value() {
            let variable = variable.clone();
            let reference = self.define_reference(variable)?;
            let mut variable = self.clone_collection(reference)?;
            variable.clear();

            Ok(variable)
        } else {
            let mut variable = variable.clone();
            variable.clear();

            Ok(variable)
        }
    }

    pub fn depth(&self) -> Depth {
        self.references.len().saturating_sub(1)
    }

    pub fn enter(&mut self) {
        self.variables.push(HashMap::new());
        self.references.push(HashMap::new());
    }

    fn remove_reference(&mut self, reference: Reference) -> KsResult<()> {
        self.remove_owner(&reference)?;
        if !self.owned(&reference)? {
            self.variable_remove(&reference)?;
        }
        Ok(())
    }

    pub fn free(&mut self, reference: &Reference) -> KsResult<()> {
        let current_depth = self.depth();
        let var_info = self.info(reference)?;

        if *var_info.depth() < current_depth {
            return Ok(());
        }

        self.variable_iter(
            *reference,
            |this, frame| this.remove_reference(frame.reference),
            |this, frame| this.remove_reference(frame.reference),
            |this, frame| this.remove_reference(frame.reference),
        )
    }

    pub fn exit(&mut self) -> KsResult<()> {
        let variables = self.variables.pop();
        if let Some(variables) = variables {
            for reference in variables.values() {
                self.free(reference)?;
            }
        }

        self.references.pop();

        Ok(())
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
        println!("Infos =====================");
        for (reference, info) in &self.infos {
            println!("{}: {:?}", reference, info);
        }

        println!("===========================");
    }
}
