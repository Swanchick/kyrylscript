use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::variable::Variable;
use crate::environment::{Depth, Owners, Reference};

#[derive(Debug)]
pub struct VarInfo {
    reference: Option<Reference>,
    depth: Depth,
    owners: Owners,
}

impl VarInfo {
    pub fn new(reference: Reference, depth: Depth) -> VarInfo {
        VarInfo {
            reference: Some(reference),
            depth,
            owners: 0,
        }
    }

    pub fn create(variable: &Variable, depth: Depth) -> KsResult<VarInfo> {
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

    pub fn reference(&self) -> KsResult<&Reference> {
        if let Some(reference) = &self.reference {
            Ok(reference)
        } else {
            Err(KsError::runtime("Variable doesn't have a reference!"))
        }
    }

    pub fn depth(&self) -> &Depth {
        &self.depth
    }

    pub fn owners(&self) -> &Owners {
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

    pub fn set_depth(&mut self, depth: Depth) {
        self.depth = depth;
    }

    pub fn set_owners(&mut self, owners: Owners) {
        self.owners = owners;
    }
}
