use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use std::collections::HashMap;

use super::types::VariableId;

pub enum Module {
    Field(VariableId),
    Module {
        variable_id: VariableId,
        fields: HashMap<String, Module>,
    },
    Root(HashMap<String, Module>),
}

impl Module {
    fn fields(&self) -> KsResult<&HashMap<String, Module>> {
        if let Self::Root(fields)
        | Self::Module {
            variable_id: _,
            fields,
        } = self
        {
            Ok(fields)
        } else {
            Err(KsError::parse(
                "Cannot get fields from the non-module state",
            ))
        }
    }

    fn fields_mut(&mut self) -> KsResult<&mut HashMap<String, Module>> {
        if let Self::Root(fields)
        | Self::Module {
            variable_id: _,
            fields,
        } = self
        {
            Ok(fields)
        } else {
            Err(KsError::parse(
                "Cannot get fields from the non-module state",
            ))
        }
    }

    pub fn len(&self) -> KsResult<usize> {
        let fields = self.fields()?;
        Ok(fields.len())
    }

    pub fn insert_module(&mut self, name: String, other: Module) -> KsResult<()> {
        let fields = self.fields_mut()?;
        fields.insert(name, other);
        Ok(())
    }

    pub fn insert_field(&mut self, name: String) -> KsResult<()> {
        let fields = self.fields_mut()?;
        let fields_len = fields.len();
        fields.insert(name, Module::Field(fields_len));
        Ok(())
    }
}
