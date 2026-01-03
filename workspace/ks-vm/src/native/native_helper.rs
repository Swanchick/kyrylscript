use ks_global::utils::ks_result::KsResult;

use crate::environment::{Environment, Reference};
use crate::variable::Variable;

pub struct NativeHelper<'a> {
    environment: &'a mut Environment,
}

impl<'a> NativeHelper<'a> {
    pub fn from(environment: &'a mut Environment) -> NativeHelper<'a> {
        NativeHelper { environment }
    }

    pub fn create_collections(&mut self, variables: Vec<Variable>) -> KsResult<Vec<Reference>> {
        let mut references: Vec<Reference> = Vec::new();

        for variable in variables {
            let reference = self.environment.define_reference(variable)?;
            references.push(reference);
        }

        Ok(references)
    }
}
