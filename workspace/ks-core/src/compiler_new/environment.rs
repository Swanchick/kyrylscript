use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use std::collections::HashMap;

use super::module::Module;
use super::types::{Pointer, VariableId};

pub struct Environment {
    functions: HashMap<String, Pointer>,
    variables: HashMap<String, VariableId>,
    modules: HashMap<VariableId, Module>,
    temporary_module: Vec<Module>,
    current: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            variables: HashMap::new(),
            modules: HashMap::new(),
            temporary_module: Vec::new(),
            current: 0,
        }
    }

    pub fn functions(self) -> HashMap<String, Pointer> {
        self.functions
    }

    pub fn define_variable(&mut self, name: String) -> KsResult<VariableId> {
        let current_count = self.current;

        self.variables.insert(name, current_count);
        self.current += 1;

        Ok(current_count)
    }

    pub fn create_module(&mut self) -> KsResult<()> {
        if let Some(module) = self.temporary_module.last() {
            let variable_id = module.len()?;
            self.temporary_module.push(Module::Module {
                variable_id,
                fields: HashMap::new(),
            });
        } else {
            self.temporary_module.push(Module::Root(HashMap::new()));
        }

        Ok(())
    }

    fn module_pop(&mut self) -> KsResult<Module> {
        if let Some(module) = self.temporary_module.pop() {
            Ok(module)
        } else {
            Err(KsError::parse("Cannot get last temporary module"))
        }
    }

    fn module_last_mut(&mut self) -> KsResult<&mut Module> {
        if let Some(module) = self.temporary_module.last_mut() {
            Ok(module)
        } else {
            Err(KsError::parse("Cannot get last temporary module"))
        }
    }

    pub fn insert_field(&mut self, name: String) -> KsResult<()> {
        let last_module = self.module_last_mut()?;
        last_module.insert_field(name)?;
        Ok(())
    }

    pub fn insert_module(&mut self, name: String) -> KsResult<()> {
        let module = self.module_pop()?;
        let last_module = self.module_last_mut()?;
        last_module.insert_module(name, module)?;
        Ok(())
    }

    pub fn temporary_modules_len(&self) -> usize {
        self.temporary_module.len()
    }

    pub fn define_module(
        &mut self,
        name: String,
        module: HashMap<String, VariableId>,
    ) -> KsResult<VariableId> {
        let variable_id = self.define_variable(name)?;
        // self.modules.insert(variable_id, HashMap::new());

        Ok(variable_id)
    }

    fn module(&mut self, module_variable_id: VariableId) -> KsResult<&HashMap<String, VariableId>> {
        // if let Some(module) = self.modules.get(&module_variable_id) {
        //     Ok(module)
        // } else {
        //     Err(KsError::parse("Cannot find module"))
        // }

        todo!()
    }

    fn module_mut(
        &mut self,
        module_variable_id: VariableId,
    ) -> KsResult<&mut HashMap<String, VariableId>> {
        // if let Some(module) = self.modules.get_mut(&module_variable_id) {
        //     Ok(module)
        // } else {
        //     Err(KsError::parse("Cannot find module"))
        // }

        todo!()
    }

    pub fn add_module_field(
        &mut self,
        module_name: &str,
        field_name: String,
    ) -> KsResult<VariableId> {
        let module_variable_id = self.variable_id(module_name)?;
        let module = self.module_mut(module_variable_id)?;
        let field_id = module.len();
        module.insert(field_name, field_id);

        Ok(field_id)
    }

    pub fn get_module_field(
        &mut self,
        module_name: &str,
        field_name: &str,
    ) -> KsResult<VariableId> {
        let module_variable_id = self.variable_id(module_name)?;
        let module = self.module(module_variable_id)?;
        if let Some(field) = module.get(field_name) {
            Ok(*field)
        } else {
            Err(KsError::parse("Cannot find field in module!"))
        }
    }

    pub fn define_function(&mut self, name: &str, pointer: Pointer) {
        self.functions.insert(name.to_string(), pointer);
    }

    pub fn variable_id(&self, name: &str) -> KsResult<VariableId> {
        if let Some(variable_id) = self.variables.get(name) {
            Ok(*variable_id)
        } else {
            Err(KsError::parse(&format!(
                "Did not find variable by this name: {}",
                name
            )))
        }
    }
}
