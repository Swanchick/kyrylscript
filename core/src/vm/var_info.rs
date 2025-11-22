use crate::global::utils::ks_error::KsError;
use crate::global::utils::ks_result::KsResult;

use super::variable::Variable;

#[derive(Debug)]
pub struct VarInfo {
    reference: Option<u64>,
    depth: usize,
    owners: usize,
}

impl VarInfo {
    pub fn new(reference: u64, depth: usize) -> VarInfo {
        VarInfo {
            reference: Some(reference),
            depth,
            owners: 0,
        }
    }

    pub fn create(variable: &Variable, depth: usize) -> KsResult<VarInfo> {
        Ok(VarInfo {
            reference: Some(variable.reference()?),
            depth,
            owners: 0,
        })
    }

    pub fn from(variable: &Variable) -> KsResult<VarInfo> {
        Ok(VarInfo {
            reference: Some(variable.reference()?),
            depth: 0,
            owners: 0,
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

    pub fn owners(&self) -> &usize {
        &self.owners
    }

    pub fn owned(&self) -> bool {
        self.owners != 0
    }

    pub fn add_owner(&mut self) {
        self.owners += 1;
    }

    pub fn remove_owner(&mut self) {
        self.owners -= 1;
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    pub fn set_owners(&mut self, owners: usize) {
        self.owners = owners;
    }
}
