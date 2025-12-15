use crate::vm::environment::Environment;
use crate::vm::variable::Variable;
use global::utils::ks_result::KsResult;

pub struct NativeHelper<'a> {
    environment: &'a mut Environment,
}

impl<'a> NativeHelper<'a> {
    pub fn from(environment: &'a mut Environment) -> NativeHelper<'a> {
        NativeHelper { environment }
    }

    pub fn create_collections(&mut self, variables: Vec<Variable>) -> KsResult<Vec<u64>> {
        let mut references: Vec<u64> = Vec::new();

        for variable in variables {
            let reference = self.environment.define_reference(variable)?;
            references.push(reference);
        }

        Ok(references)
    }
}
