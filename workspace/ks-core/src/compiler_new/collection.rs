use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use std::collections::HashMap;

use super::types::VariableId;

pub enum Collection {
    Field(VariableId),
    Module {
        variable_id: Option<VariableId>,
        fields: HashMap<String, Collection>,
    },
    List {
        children: Option<Box<Collection>>,
    },
    Tuple {
        children: Vec<Collection>,
    },
}

impl Collection {
    fn fields(&self) -> KsResult<&HashMap<String, Collection>> {
        if let Self::Module {
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

    fn fields_mut(&mut self) -> KsResult<&mut HashMap<String, Collection>> {
        if let Self::Module {
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

    pub fn insert_module(&mut self, name: String, other: Collection) -> KsResult<()> {
        let fields = self.fields_mut()?;
        fields.insert(name, other);
        Ok(())
    }

    pub fn insert_field(&mut self, name: String) -> KsResult<()> {
        let fields = self.fields_mut()?;
        let fields_len = fields.len();
        fields.insert(name, Collection::Field(fields_len));
        Ok(())
    }
}
