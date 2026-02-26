use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use std::collections::HashMap;

use super::types::VariableId;

pub enum Slot {
    Variable(VariableId),
    Module {
        variable_id: VariableId,
        fields: HashMap<String, Slot>,
    },
    List {
        variable_id: VariableId,
        children: Option<Box<Slot>>,
    },
    Tuple {
        variable_id: VariableId,
        children: Vec<Slot>,
    },
}

impl Slot {
    fn fields(&self) -> KsResult<&HashMap<String, Slot>> {
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

    fn fields_mut(&mut self) -> KsResult<&mut HashMap<String, Slot>> {
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

    pub fn insert_module(&mut self, name: String, other: Slot) -> KsResult<()> {
        let fields = self.fields_mut()?;
        fields.insert(name, other);
        Ok(())
    }

    pub fn insert_variable(&mut self, name: String) -> KsResult<()> {
        let fields = self.fields_mut()?;
        let fields_len = fields.len();
        fields.insert(name, Slot::Variable(fields_len));
        Ok(())
    }

    pub fn variable_id(&self) -> &VariableId {
        match self {
            Self::Variable(variable_id)
            | Self::Module {
                variable_id,
                fields: _,
            }
            | Self::List {
                variable_id,
                children: _,
            }
            | Self::Tuple {
                variable_id,
                children: _,
            } => variable_id,
        }
    }
}
