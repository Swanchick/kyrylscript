use crate::global::utils::ks_result::KsResult;
use crate::global::utils::ks_error::KsError;

use super::variable::Variable;

#[derive(Debug)]
pub struct VarInfo {
    reference: Option<u64>,
    depth: usize,
    owner: usize,
}

impl VarInfo {
    pub fn new(reference: u64, depth: usize, owner: usize) -> VarInfo {
        VarInfo {
            reference: Some(reference),
            depth,
            owner,
        }
    }

    pub fn from(variable: &Variable) -> KsResult<VarInfo> {
        Ok(VarInfo {
            reference: Some(variable.reference()?),
            depth: variable.depth(),
            owner: variable.owners(),
        })
    }

    pub fn reference(&self) -> KsResult<&u64> {
        if let Some(reference) = &self.reference {
            Ok(reference)
        } else {
            Err(KsError::runtime("Variable doesn't have a reference!"))
        }
    }

    pub fn depth(&self) -> &usize {
        &self.depth
    }

    pub fn owner(&self) -> &usize {
        &self.owner
    }
}
