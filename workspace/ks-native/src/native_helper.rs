use ks_global::utils::ks_result::KsResult;

use crate::{native_environment::NativeEnvironment, native_types::NativeType};

pub struct NativeHelper<'a, T>
where
    T: NativeEnvironment,
{
    environment: &'a mut T,
}

impl<'a, T: NativeEnvironment> NativeHelper<'a, T> {
    pub fn from(environment: &'a mut T) -> NativeHelper<'a, T> {
        NativeHelper { environment }
    }

    pub fn create_collections(&mut self, variables: Vec<NativeType>) -> KsResult<Vec<u64>> {
        let mut references: Vec<u64> = Vec::new();

        for variable in variables {
            let reference = self.environment.define_reference(variable)?;
            references.push(reference);
        }

        Ok(references)
    }
}
